pub mod build_project_params;
pub mod build_project_result;
pub mod build_project_result_holder;
pub mod build_project_result_stub;
mod content_document_rendering_context;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;
use dashmap::DashMap;
use log::debug;
use log::info;
use rayon::iter::IntoParallelRefIterator as _;
use rayon::iter::ParallelIterator as _;
use rhai::Dynamic;
use syntect::parsing::SyntaxSet;

use crate::asset_manager::AssetManager;
use crate::author_resolve_result::AuthorResolveResult;
use crate::build_project::build_project_params::BuildProjectParams;
use crate::build_project::build_project_result_stub::BuildProjectResultStub;
use crate::build_project::content_document_rendering_context::ContentDocumentRenderingContext;
use crate::build_timer::BuildTimer;
use crate::content_document::ContentDocument;
use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_collection::ContentDocumentCollection;
use crate::content_document_collection_ranked::ContentDocumentCollectionRanked;
use crate::content_document_component_context::ContentDocumentComponentContext;
use crate::content_document_front_matter::ContentDocumentFrontMatter;
use crate::content_document_in_collection::ContentDocumentInCollection;
use crate::content_document_linker::ContentDocumentLinker;
use crate::content_document_reference::ContentDocumentReference;
use crate::content_document_source::ContentDocumentSource;
use crate::document_error_collection::DocumentErrorCollection;
use crate::eval_content_document_mdast::eval_content_document_mdast;
use crate::filesystem::Filesystem as _;
use crate::filesystem::memory::Memory;
use crate::find_front_matter_in_mdast::find_front_matter_in_mdast;
use crate::find_table_of_contents_in_mdast::find_table_of_contents_in_mdast;
use crate::generate_sitemap::create_sitemap;
use crate::string_to_mdast::string_to_mdast;

fn render_document<'render>(
    ContentDocumentRenderingContext {
        asset_path_renderer,
        authors,
        available_authors,
        available_collections,
        content_document:
            ContentDocument {
                mdast,
                reference:
                    reference @ ContentDocumentReference {
                        basename_path: _,
                        front_matter,
                        generated_page_base_path: _,
                    },
            },
        content_document_collections_ranked,
        content_document_linker,
        esbuild_metafile,
        is_watching,
        rhai_template_renderer,
        syntax_set,
    }: ContentDocumentRenderingContext<'render>,
) -> Result<String> {
    let component_context = ContentDocumentComponentContext {
        asset_manager: AssetManager::from_esbuild_metafile(esbuild_metafile, asset_path_renderer),
        authors: authors.clone(),
        available_authors,
        available_collections,
        content_document_collections_ranked,
        content_document_linker,
        front_matter: front_matter.clone(),
        is_watching,
        reference: reference.clone(),
        table_of_contents: None,
    };

    let table_of_contents = find_table_of_contents_in_mdast(
        mdast,
        &component_context,
        rhai_template_renderer,
        syntax_set,
    )?;

    let component_context_with_toc = component_context.with_table_of_contents(table_of_contents);

    let layout_content = eval_content_document_mdast(
        mdast,
        &component_context_with_toc,
        rhai_template_renderer,
        syntax_set,
    )?;

    rhai_template_renderer.render(
        &front_matter.layout,
        component_context_with_toc.clone(),
        Dynamic::from_map(front_matter.props.clone()),
        layout_content.into(),
    )
}

pub async fn build_project(
    BuildProjectParams {
        asset_path_renderer,
        authors,
        esbuild_metafile,
        generated_page_base_path,
        generate_sitemap,
        is_watching,
        rhai_template_renderer,
        source_filesystem,
    }: BuildProjectParams,
) -> Result<BuildProjectResultStub> {
    info!(
        "Building project in {}...",
        source_filesystem.base_directory.display()
    );
    info!("Processing content files...");

    let _build_timer = BuildTimer::default();
    let error_collection: DocumentErrorCollection = Default::default();
    let memory_filesystem = Arc::new(Memory::default());
    let syntax_set = SyntaxSet::load_defaults_newlines();

    let mut content_document_basename_by_id: HashMap<String, ContentDocumentBasename> =
        HashMap::new();
    let mut content_document_by_basename: HashMap<
        ContentDocumentBasename,
        ContentDocumentReference,
    > = HashMap::new();
    let mut content_document_collections: HashMap<String, ContentDocumentCollection> =
        HashMap::new();
    let mut content_document_collections_ranked: HashMap<String, ContentDocumentCollectionRanked> =
        HashMap::new();
    let mut content_document_list: Vec<ContentDocument> = Vec::new();
    let mut content_document_sources: BTreeMap<ContentDocumentBasename, ContentDocumentSource> =
        Default::default();

    for file in source_filesystem.read_project_files().await? {
        if file.kind.is_content() {
            let mdast = string_to_mdast(&file.contents)?;
            let front_matter: ContentDocumentFrontMatter = find_front_matter_in_mdast(&mdast)?
                .ok_or_else(|| {
                    anyhow!("No front matter found in file: {:?}", file.relative_path)
                })?;

            let basename_path = file.get_stem_path_relative_to(&PathBuf::from("content"));
            let basename: ContentDocumentBasename = basename_path.clone().into();
            let content_document_reference = ContentDocumentReference {
                basename_path,
                front_matter: front_matter.clone(),
                generated_page_base_path: generated_page_base_path.clone(),
            };

            if let Some(id) = &front_matter.id {
                if content_document_basename_by_id.contains_key(id) {
                    error_collection.register_error(
                        content_document_reference.basename().to_string(),
                        anyhow!("Duplicate document id: #{id} in '{basename}'"),
                    );
                }

                content_document_basename_by_id.insert(id.clone(), basename.clone());
            }

            content_document_by_basename
                .insert(basename.clone(), content_document_reference.clone());
            content_document_list.push(ContentDocument {
                mdast: mdast.clone(),
                reference: content_document_reference.clone(),
            });

            if content_document_reference.front_matter.render {
                let relative_path = format!("{basename}.md");

                content_document_sources.insert(
                    basename,
                    ContentDocumentSource {
                        file_entry: file,
                        mdast,
                        reference: content_document_reference,
                        relative_path,
                    },
                );
            }
        }
    }

    // Validate before/after/parent documents in collections
    for reference in content_document_by_basename.values() {
        // Validate primary collections
        if let Some(primary_collection) = &reference.front_matter.primary_collection
            && !reference
                .front_matter
                .collections
                .placements
                .iter()
                .any(|placement| placement.name == *primary_collection)
        {
            error_collection.register_error(
                reference.basename().to_string(),
                anyhow!(
                    "Document does belong to the collection it claims to be it's primary collection"
                ),
            );
        }

        for collection in &reference.front_matter.collections.placements {
            if let Some(after) = &collection.after
                && !content_document_by_basename.contains_key(after)
            {
                error_collection.register_error(
                    reference.basename().to_string(),
                    anyhow!("Succeeding document does not exist: '{after}'"),
                );
            }

            if let Some(parent) = &collection.parent
                && !content_document_by_basename.contains_key(parent)
            {
                error_collection.register_error(
                    reference.basename().to_string(),
                    anyhow!("Parent document does not exist: '{parent}'"),
                );
            }

            content_document_collections
                .entry(collection.name.clone())
                .or_insert_with(|| ContentDocumentCollection {
                    documents: Default::default(),
                    name: collection.name.clone(),
                })
                .documents
                .push(ContentDocumentInCollection {
                    collection_placement: collection.clone(),
                    reference: reference.clone(),
                })
        }
    }

    for content_document_collection in content_document_collections.values() {
        let content_document_collection_ranked: ContentDocumentCollectionRanked =
            content_document_collection.clone().try_into()?;

        content_document_collections_ranked.insert(
            content_document_collection.name.clone(),
            content_document_collection_ranked,
        );
    }

    if !error_collection.is_empty() {
        return Err(anyhow!("{error_collection}"));
    }

    let authors_arc = Arc::new(authors);

    let available_collections_arc: Arc<HashSet<String>> = Arc::new(
        content_document_collections
            .keys()
            .map(|key| key.to_string())
            .collect::<HashSet<String>>(),
    );
    let content_document_reference_collection_dashmap: DashMap<String, ContentDocumentReference> =
        Default::default();
    let content_document_basename_by_id_arc = Arc::new(content_document_basename_by_id);
    let content_document_by_basename_arc = Arc::new(content_document_by_basename);
    let content_document_collections_ranked_arc = Arc::new(content_document_collections_ranked);
    let content_document_linker = ContentDocumentLinker {
        content_document_basename_by_id: content_document_basename_by_id_arc.clone(),
        content_document_by_basename: content_document_by_basename_arc.clone(),
    };

    content_document_list
        .par_iter()
        .filter(|content_document| {
            if !content_document.reference.front_matter.render {
                debug!(
                    "Document will not be rendered: {}",
                    content_document.reference.basename()
                );

                false
            } else {
                true
            }
        })
        .for_each(|content_document| {
            let AuthorResolveResult {
                found_authors,
                missing_authors,
            } = authors_arc.resolve(&content_document.reference.front_matter.authors);

            for author_name in &missing_authors {
                error_collection.register_error(
                    content_document.reference.basename().to_string(),
                    anyhow!("Author does not exist: '{author_name}'"),
                );
            }

            if !missing_authors.is_empty() {
                return;
            }

            match render_document(ContentDocumentRenderingContext {
                asset_path_renderer: asset_path_renderer.clone(),
                authors: found_authors,
                available_authors: authors_arc.clone(),
                available_collections: available_collections_arc.clone(),
                esbuild_metafile: esbuild_metafile.clone(),
                is_watching,
                content_document,
                content_document_collections_ranked: content_document_collections_ranked_arc
                    .clone(),
                content_document_linker: content_document_linker.clone(),
                rhai_template_renderer: &rhai_template_renderer,
                syntax_set: &syntax_set,
            }) {
                Ok(processed_file) => {
                    match content_document.reference.target_file_relative_path() {
                        Ok(relative_path) => {
                            if let Err(err) = memory_filesystem
                                .set_file_contents_sync(&relative_path, &processed_file)
                            {
                                error_collection.register_error(
                                    content_document.reference.basename().to_string(),
                                    err,
                                );
                            } else {
                                content_document_reference_collection_dashmap.insert(
                                    relative_path.display().to_string(),
                                    content_document.reference.clone(),
                                );
                            }
                        }
                        Err(err) => {
                            error_collection.register_error(
                                content_document.reference.basename().to_string(),
                                anyhow!(err),
                            );
                        }
                    }
                }
                Err(err) => error_collection
                    .register_error(content_document.reference.basename().to_string(), err),
            }
        });

    if generate_sitemap {
        info!("Building sitemap");

        match create_sitemap(
            content_document_by_basename_arc
                .values()
                .filter(|content_document| content_document.front_matter.render),
        ) {
            Ok(sitemap) => {
                if let Err(err) =
                    memory_filesystem.set_file_contents_sync(Path::new("sitemap.xml"), &sitemap)
                {
                    error_collection.register_error("sitemap.xml".to_string(), err);
                }
            }
            Err(err) => {
                error_collection.register_error("sitemap.xml".to_string(), err);
            }
        }
    }

    if error_collection.is_empty() {
        Ok(BuildProjectResultStub {
            esbuild_metafile,
            content_document_linker,
            content_document_sources: Arc::new(content_document_sources),
            memory_filesystem,
        })
    } else {
        Err(anyhow!("{error_collection}"))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use anyhow::Result;
    use anyhow::anyhow;
    use tempfile::tempdir;

    use super::build_project;
    use crate::asset_path_renderer::AssetPathRenderer;
    use crate::build_authors::build_authors;
    use crate::build_project::build_project_params::BuildProjectParams;
    use crate::build_project::build_project_result_stub::BuildProjectResultStub;
    use crate::compile_shortcodes::compile_shortcodes;
    use crate::filesystem::Filesystem as _;
    use crate::filesystem::read_file_contents_result::ReadFileContentsResult;
    use crate::filesystem::storage::Storage;

    const LAYOUT_MINIMAL: &str = r#"
fn template(context, props, content) {
  component {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <title>Test</title>
      </head>
      <body>
        <PrimaryNavigation>nav</PrimaryNavigation>
        {content}
        <ul>
          {
            let ret = [];

            for author in context.authors {
              ret.push(component {
                <li>{author.data.name}</li>
              });
            }

            ret
          }
        </ul>
      </body>
    </html>
  }
}
"#;

    const PRIMARY_NAVIGATION: &str = r#"
fn template(context, props, content) {
  component {
    <nav>{content}</nav>
  }
}
"#;

    async fn build(
        files: &[(&str, &str)],
        generate_sitemap: bool,
    ) -> Result<BuildProjectResultStub> {
        let directory = tempdir()?;
        let source_filesystem = Arc::new(Storage {
            base_directory: directory.path().to_path_buf(),
        });

        for (relative_path, contents) in files {
            source_filesystem
                .set_file_contents(Path::new(relative_path), contents)
                .await?;
        }

        let rhai_template_renderer = compile_shortcodes(source_filesystem.clone()).await?;
        let authors = build_authors(source_filesystem.clone()).await?;

        build_project(BuildProjectParams {
            asset_path_renderer: AssetPathRenderer {
                base_path: "/".to_string(),
            },
            authors,
            esbuild_metafile: Default::default(),
            generated_page_base_path: "/".to_string(),
            generate_sitemap,
            is_watching: false,
            rhai_template_renderer,
            source_filesystem,
        })
        .await
    }

    async fn read(result: &BuildProjectResultStub, relative_path: &str) -> Result<String> {
        match result
            .memory_filesystem
            .read_file_contents(Path::new(relative_path))
            .await?
        {
            ReadFileContentsResult::Found { contents } => Ok(contents),
            ReadFileContentsResult::Directory => Err(anyhow!("{relative_path} is a directory")),
            ReadFileContentsResult::NotFound => Err(anyhow!("{relative_path} was not generated")),
        }
    }

    #[tokio::test]
    async fn renders_content_documents_to_their_target_paths() -> Result<()> {
        let result = build(
            &[
                ("shortcodes/LayoutMinimal.rhai", LAYOUT_MINIMAL),
                ("shortcodes/PrimaryNavigation.rhai", PRIMARY_NAVIGATION),
                ("authors/alice.toml", "name = \"Alice\""),
                (
                    "content/index.md",
                    "+++\ndescription = \"Home\"\nlayout = \"LayoutMinimal\"\ntitle = \"Home\"\nauthors = [\"alice\"]\n+++\n\n# Hello\n\nBody text.\n",
                ),
                (
                    "content/docs/index.md",
                    "+++\ndescription = \"Docs\"\nlayout = \"LayoutMinimal\"\ntitle = \"Docs\"\n\n[[collection]]\nname = \"docs\"\n+++\n\nDocs index.\n",
                ),
                (
                    "content/docs/page.md",
                    "+++\ndescription = \"Page\"\nlayout = \"LayoutMinimal\"\ntitle = \"Page\"\n\n[[collection]]\nname = \"docs\"\nafter = \"docs/index\"\n+++\n\nPage body.\n",
                ),
            ],
            false,
        )
        .await?;

        let home = read(&result, "index.html").await?;

        assert!(home.contains("<!DOCTYPE html>"));
        assert!(home.contains("<nav>nav</nav>"));
        assert!(home.contains("<li>Alice</li>"));

        read(&result, "docs/index.html").await?;
        read(&result, "docs/page/index.html").await?;

        assert_eq!(result.content_document_sources.len(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn generates_sitemap_with_canonical_links_when_requested() -> Result<()> {
        let result = build(
            &[
                ("shortcodes/LayoutMinimal.rhai", LAYOUT_MINIMAL),
                ("shortcodes/PrimaryNavigation.rhai", PRIMARY_NAVIGATION),
                (
                    "content/index.md",
                    "+++\ndescription = \"Home\"\nlayout = \"LayoutMinimal\"\ntitle = \"Home\"\n+++\n\nHome.\n",
                ),
            ],
            true,
        )
        .await?;

        let sitemap = read(&result, "sitemap.xml").await?;

        assert!(sitemap.contains("<urlset"));
        assert!(sitemap.contains("<loc>/</loc>"));

        Ok(())
    }

    const LAYOUT_RICH: &str = r#"
fn template(context, props, content) {
  component {
    <html>
      <head><title>{context.front_matter.title}</title></head>
      <body>
        <p>{context.front_matter.description}</p>
        <span>{context.reference.canonical_link}</span>
        <span>{context.reference.basename}</span>
        <span>reftitle:{context.reference.front_matter.title}</span>
        <span>watching:{context.is_watching}</span>
        <span>props:{context.front_matter.props.len()}</span>
        <span>authors:{context.available_authors.len()}</span>
        {if context.front_matter.render { "<i>is-rendered</i>" } else { "" }}
        {
          let names = "";

          for author in context.authors {
            names += "<b>" + author.basename + "=" + author.data.name + "</b>";
          }

          names
        }
        <ul>
        {
          let toc = "";

          for heading in context.table_of_contents.headings {
            toc += "<li>" + heading.id + ":" + heading.content + ":" + heading.depth + "</li>";
          }

          toc
        }
        </ul>
        {
          let docs = context.collection("docs");
          let heading = "<h2>" + docs.name + "</h2>";

          heading + render_hierarchy(docs.hierarchy, |node, level, children| {
            "<li>" + node.reference.basename + " in " + node.collection_name + " kids " + node.children.len() + children + "</li>"
          })
        }
        {content}
      </body>
    </html>
  }
}
"#;

    const LAYOUT_PLAIN: &str = r#"
fn template(context, props, content) {
  component {
    <html>{content}</html>
  }
}
"#;

    #[tokio::test]
    async fn rich_layout_exercises_template_accessors_and_hierarchy() -> Result<()> {
        let result = build(
            &[
                ("shortcodes/LayoutRich.rhai", LAYOUT_RICH),
                ("shortcodes/LayoutPlain.rhai", LAYOUT_PLAIN),
                ("authors/alice.toml", "name = \"Alice\""),
                (
                    "content/index.md",
                    "+++\ndescription = \"Welcome home\"\nlayout = \"LayoutRich\"\ntitle = \"Home Page\"\nauthors = [\"alice\"]\n+++\n\n# Section One\n\nHome body.\n",
                ),
                (
                    "content/docs/index.md",
                    "+++\ndescription = \"Docs\"\nlayout = \"LayoutPlain\"\ntitle = \"Docs\"\n\n[[collection]]\nname = \"docs\"\n+++\n\nDocs index.\n",
                ),
                (
                    "content/docs/page.md",
                    "+++\ndescription = \"Page\"\nlayout = \"LayoutPlain\"\ntitle = \"Page\"\n\n[[collection]]\nname = \"docs\"\nafter = \"docs/index\"\n+++\n\nPage body.\n",
                ),
            ],
            false,
        )
        .await?;

        let home = read(&result, "index.html").await?;

        assert!(home.contains("<title>Home Page</title>"));
        assert!(home.contains("Welcome home"));
        assert!(home.contains("<i>is-rendered</i>"));
        assert!(home.contains("reftitle:Home Page"));
        assert!(home.contains("watching:false"));
        assert!(home.contains("authors:1"));
        assert!(home.contains("<b>alice=Alice</b>"));
        assert!(home.contains("section-one:Section One:1"));
        assert!(home.contains("<h2>docs</h2>"));
        assert!(home.contains("in docs kids"));

        Ok(())
    }

    #[tokio::test]
    async fn renders_markdown_mdx_component_and_expression() -> Result<()> {
        let result = build(
            &[
                ("shortcodes/LayoutMinimal.rhai", LAYOUT_MINIMAL),
                ("shortcodes/PrimaryNavigation.rhai", PRIMARY_NAVIGATION),
                (
                    "content/index.md",
                    "+++\ndescription = \"Home\"\nlayout = \"LayoutMinimal\"\ntitle = \"Home\"\n+++\n\nValue {40 + 2}\n\n<PrimaryNavigation>\ninner\n</PrimaryNavigation>\n",
                ),
            ],
            false,
        )
        .await?;

        let home = read(&result, "index.html").await?;

        assert!(home.contains("<p>Value 42</p>"));
        assert!(home.contains("<nav><p>inner</p></nav>"));

        Ok(())
    }

    #[tokio::test]
    async fn errors_on_duplicate_document_id() -> Result<()> {
        let front_matter = |title: &str| {
            format!(
                "+++\ndescription = \"d\"\nid = \"dup\"\nlayout = \"LayoutMinimal\"\ntitle = \"{title}\"\n+++\n\nBody.\n"
            )
        };

        let outcome = build(
            &[
                ("content/a.md", &front_matter("A")),
                ("content/b.md", &front_matter("B")),
            ],
            false,
        )
        .await;

        assert!(
            outcome.is_err_and(|error| error.to_string().contains("Duplicate document id: #dup"))
        );

        Ok(())
    }

    #[tokio::test]
    async fn errors_when_a_referenced_author_does_not_exist() -> Result<()> {
        let outcome = build(
            &[(
                "content/a.md",
                "+++\ndescription = \"d\"\nlayout = \"LayoutMinimal\"\ntitle = \"A\"\nauthors = [\"ghost\"]\n+++\n\nBody.\n",
            )],
            false,
        )
        .await;

        assert!(
            outcome.is_err_and(|error| {
                error.to_string().contains("Author does not exist: 'ghost'")
            })
        );

        Ok(())
    }
}
