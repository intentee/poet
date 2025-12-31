pub mod build_blogs_params;

use std::path::PathBuf;

use anyhow::Result;
use anyhow::anyhow;

use crate::author_resolve_result::AuthorResolveResult;
use crate::blog_post_basename::BlogPostBasename;
use crate::blog_post_front_matter::BlogPostFrontMatter;
use crate::build_project::build_blogs::build_blogs_params::BuildBlogsParams;
use crate::document_error_collection::DocumentErrorCollection;
use crate::filesystem::Filesystem as _;
use crate::find_front_matter_in_mdast::find_front_matter_in_mdast;
use crate::string_to_mdast::string_to_mdast;

pub async fn build_blogs(
    BuildBlogsParams {
        authors,
        source_filesystem,
    }: BuildBlogsParams,
) -> Result<()> {
    let error_collection: DocumentErrorCollection = Default::default();

    for file in source_filesystem.read_project_files().await? {
        if file.kind.is_blog_post() {
            let mdast = string_to_mdast(&file.contents)?;

            let front_matter: BlogPostFrontMatter = find_front_matter_in_mdast(&mdast)?
                .ok_or_else(|| {
                    anyhow!("No front matter found in file: {:?}", file.relative_path)
                })?;

            let basename_path = file.get_stem_path_relative_to(&PathBuf::from("blogs"));
            let basename: BlogPostBasename = basename_path.clone().into();

            let AuthorResolveResult {
                found_authors: _,
                missing_authors,
            } = authors.resolve(&front_matter.authors);

            for author_name in &missing_authors {
                error_collection.register_error(
                    basename.to_string(),
                    anyhow!("Author does not exist: '{author_name}'"),
                );
            }

            if !missing_authors.is_empty() {
                continue;
            }
        }
    }

    if error_collection.is_empty() {
        Ok(())
    } else {
        Err(anyhow!("{error_collection}"))
    }
}
