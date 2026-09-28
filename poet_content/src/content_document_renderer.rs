use std::sync::Arc;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use poet_assets::asset_manager::AssetManager;
use poet_assets::asset_path_renderer::AssetPathRenderer;
use poet_mdx::document_error_collection::DocumentErrorCollection;
use rayon::iter::IntoParallelRefIterator as _;
use rayon::iter::ParallelIterator as _;
use rhai::Dynamic;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;
use syntect::parsing::SyntaxSet;

use crate::author::Author;
use crate::author_resolve_result::AuthorResolveResult;
use crate::content_document_component_context::ContentDocumentComponentContext;
use crate::content_document_context::ContentDocumentContext;
use crate::content_document_evaluator::ContentDocumentEvaluator;
use crate::content_document_source::ContentDocumentSource;
use crate::content_error::ContentError;
use crate::content_site_context::ContentSiteContext;
use crate::generated_file::GeneratedFile;
use crate::generated_file_kind::GeneratedFileKind;
use crate::table_of_contents_state::TableOfContentsState;

pub struct ContentDocumentRenderer<'renderer> {
    pub asset_path_renderer: &'renderer AssetPathRenderer,
    pub esbuild_metafile: &'renderer Arc<EsbuildMetafile>,
    pub rhai_template_renderer: &'renderer RhaiTemplateRenderer,
    pub site_context: &'renderer ContentSiteContext,
    pub syntax_set: &'renderer SyntaxSet,
}

impl ContentDocumentRenderer<'_> {
    pub fn render_page(
        &self,
        ContentDocumentSource {
            mdast, reference, ..
        }: &ContentDocumentSource,
        authors: Vec<Author>,
    ) -> Result<String, ContentError> {
        let component_context = ContentDocumentComponentContext {
            asset_manager: AssetManager::from_esbuild_metafile(
                self.esbuild_metafile.clone(),
                self.asset_path_renderer.clone(),
            ),
            document: ContentDocumentContext {
                authors,
                reference: reference.clone(),
            },
            site: self.site_context.clone(),
            table_of_contents: TableOfContentsState::BeingCollected,
        };
        let table_of_contents = self
            .evaluator(&component_context)
            .table_of_contents(mdast)?;
        let component_context = component_context.with_table_of_contents(table_of_contents);
        let layout_content = self.evaluator(&component_context).eval(mdast)?;

        self.rhai_template_renderer
            .render(
                &reference.front_matter.layout,
                component_context,
                Dynamic::from_map(reference.front_matter.props.clone()),
                Dynamic::from(layout_content),
            )
            .map_err(|source| ContentError::RenderLayout {
                layout: reference.front_matter.layout.clone(),
                source,
            })
    }

    pub fn render_pages(
        &self,
        content_document_sources: &[ContentDocumentSource],
    ) -> Result<Vec<GeneratedFile>, ContentError> {
        let document_errors = DocumentErrorCollection::default();
        let generated_pages: Vec<GeneratedFile> = content_document_sources
            .par_iter()
            .filter(|content_document_source| content_document_source.reference.front_matter.render)
            .filter_map(|content_document_source| {
                let basename = content_document_source.reference.basename();
                let AuthorResolveResult {
                    found_authors,
                    missing_authors,
                } = self
                    .site_context
                    .available_authors
                    .resolve(&content_document_source.reference.front_matter.authors);

                if !missing_authors.is_empty() {
                    for author_name in missing_authors {
                        document_errors.register_error(
                            basename.to_string(),
                            ContentError::AuthorNotFound {
                                author_name,
                                basename: basename.clone(),
                            },
                        );
                    }

                    return None;
                }

                match self.render_page(content_document_source, found_authors) {
                    Ok(page) => Some(GeneratedFile {
                        contents: page,
                        kind: GeneratedFileKind::Page,
                        relative_path: content_document_source
                            .reference
                            .target_file_relative_path(),
                    }),
                    Err(render_error) => {
                        document_errors.register_error(basename.to_string(), render_error);

                        None
                    }
                }
            })
            .collect();

        if document_errors.is_empty() {
            Ok(generated_pages)
        } else {
            Err(ContentError::InvalidDocuments(document_errors))
        }
    }

    const fn evaluator<'evaluator>(
        &'evaluator self,
        component_context: &'evaluator ContentDocumentComponentContext,
    ) -> ContentDocumentEvaluator<'evaluator> {
        ContentDocumentEvaluator {
            component_context,
            rhai_template_renderer: self.rhai_template_renderer,
            syntax_set: self.syntax_set,
        }
    }
}
