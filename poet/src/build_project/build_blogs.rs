use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;
use log::info;

use crate::blog::Blog;
use crate::blog_collection::BlogCollection;
use crate::blog_config::BlogConfig;
use crate::blog_name::BlogName;
use crate::blog_post::BlogPost;
use crate::blog_post_collection::BlogPostCollection;
use crate::blog_post_front_matter::BlogPostFrontMatter;
use crate::document_error_collection::DocumentErrorCollection;
use crate::filesystem::Filesystem as _;
use crate::filesystem::storage::Storage;
use crate::find_front_matter_in_mdast::find_front_matter_in_mdast;
use crate::string_to_mdast::string_to_mdast;

pub async fn build_blogs(source_filesystem: Arc<Storage>) -> Result<BlogCollection> {
    info!(
        "Building blogs in {}...",
        source_filesystem.base_directory.display()
    );
    info!("Processing blog files...");

    let error_collection: DocumentErrorCollection = Default::default();
    let mut blog_collection = BlogCollection::default();

    for file in source_filesystem.read_blog_config_files().await? {
        let config: BlogConfig = match toml::from_str(&file.contents) {
            Ok(config) => config,
            Err(err) => {
                error_collection.register_error(
                    file.relative_path.display().to_string(),
                    anyhow!("Failed to parse blog config file: {err:#?}"),
                );

                continue;
            }
        };

        let blog_name_path = file.get_stem_path_relative_to(&PathBuf::from("blogs"));
        let blog_name: BlogName = blog_name_path.into();

        let mut posts = BlogPostCollection::default();
        let blog_posts_dir = blog_name.relative_blog_directory();

        for post_file in source_filesystem
            .read_blog_posts_from_blog(&blog_name)
            .await?
        {
            let mdast = match string_to_mdast(&post_file.contents) {
                Ok(mdast) => mdast,
                Err(err) => {
                    error_collection.register_error(
                        post_file.relative_path.display().to_string(),
                        anyhow!("Failed to parse blog post: {err:#?}"),
                    );

                    continue;
                }
            };

            let front_matter: BlogPostFrontMatter = match find_front_matter_in_mdast(&mdast) {
                Ok(Some(front_matter)) => front_matter,
                Ok(None) => {
                    error_collection.register_error(
                        post_file.relative_path.display().to_string(),
                        anyhow!("No front matter found in blog post"),
                    );

                    continue;
                }
                Err(err) => {
                    error_collection.register_error(
                        post_file.relative_path.display().to_string(),
                        anyhow!("Failed to parse blog post front matter: {err:#?}"),
                    );

                    continue;
                }
            };

            let basename = post_file.get_stem_path_relative_to(&blog_posts_dir).into();

            posts.insert(BlogPost {
                basename,
                blog_name: blog_name.clone(),
                front_matter,
            });
        }

        blog_collection.insert(Blog {
            name: blog_name,
            config,
            posts,
        });
    }

    if error_collection.is_empty() {
        Ok(blog_collection)
    } else {
        Err(anyhow!("{error_collection}"))
    }
}
