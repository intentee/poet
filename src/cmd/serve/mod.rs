mod app_data;

use std::net::SocketAddr;
use std::path::PathBuf;

use actix_web::web::Data;
use anyhow::Result;
use indoc::formatdoc;
use anyhow::anyhow;
use async_trait::async_trait;
use clap::Parser;
use log::info;

use crate::app_dir_desktop_entry::AppDirDesktopEntry;
use crate::asset_path_renderer::AssetPathRenderer;
use crate::build_project::build_project;
use crate::build_project::build_project_params::BuildProjectParams;
use crate::build_project::build_project_result::BuildProjectResult;
use crate::build_prompt_document_controller_collection::build_prompt_document_controller_collection;
use crate::build_prompt_document_controller_collection::build_prompt_document_controller_collection_params::BuildPromptControllerCollectionParams;
use crate::cmd::builds_project::BuildsProject;
use crate::cmd::handler::Handler;
use crate::cmd::serve::app_data::AppData;
use crate::cmd::value_parser::parse_socket_addr;
use crate::cmd::value_parser::validate_is_directory;
use crate::compile_shortcodes::compile_shortcodes;
use crate::filesystem::Filesystem;
use crate::filesystem_http_route_index::FilesystemHttpRouteIndex;
use crate::mcp::jsonrpc::implementation::Implementation;
use crate::read_esbuild_metafile_or_default::read_esbuild_metafile_or_default;
use crate::search_index::SearchIndex;
use crate::search_index_reader::SearchIndexReader;

#[derive(Parser)]
pub struct Serve {
    #[arg(long, default_value="127.0.0.1:8070", value_parser = parse_socket_addr)]
    addr: SocketAddr,

    #[arg(value_parser = validate_is_directory)]
    app_dir: PathBuf,

    #[arg(long)]
    public_path: String,
}

impl Serve {
    fn app_dir_name_stem(&self) -> Result<String> {
        self.app_dir
            .file_stem()
            .ok_or_else(|| anyhow!("Unable to get AppDir name stem"))
            .map(|os_str| os_str.to_string_lossy().to_string())
    }
}

impl BuildsProject for Serve {
    fn source_directory(&self) -> PathBuf {
        self.app_dir.clone()
    }
}

#[async_trait(?Send)]
impl Handler for Serve {
    async fn handle(&self) -> Result<()> {
        let app_name = self.app_dir_name_stem()?;
        let asset_path_renderer = AssetPathRenderer {
            base_path: self.public_path.clone(),
        };
        let source_filesystem = self.source_filesystem();
        let rhai_template_renderer = compile_shortcodes(source_filesystem.clone()).await?;
        let app_dir_desktop_entry = AppDirDesktopEntry::parse(
            &source_filesystem
                .read_file_contents_string(&PathBuf::from(format!(
                    "{}.desktop",
                    app_name.to_lowercase()
                )))
                .await?,
        )?;

        info!(
            "{}",
            formatdoc! {
                r#"
                    Site details:
                    ├── name: {name}
                    ├── title: {title}
                    ├── generated with Poet version: {poet_version}
                    └── version: {site_version}
                "#,
                name = app_dir_desktop_entry.name,
                poet_version = app_dir_desktop_entry.poet_version,
                site_version = app_dir_desktop_entry.site_version,
                title = app_dir_desktop_entry.title,
            }
        );

        let server_info = Implementation {
            name: app_dir_desktop_entry.name.clone(),
            title: Some(app_dir_desktop_entry.title.clone()),
            version: app_dir_desktop_entry.site_version.clone(),
        };

        let BuildProjectResult {
            content_document_linker,
            content_document_sources,
            esbuild_metafile,
            memory_filesystem,
            ..
        } = build_project(BuildProjectParams {
            asset_path_renderer: asset_path_renderer.clone(),
            esbuild_metafile: read_esbuild_metafile_or_default(source_filesystem.clone()).await?,
            generated_page_base_path: self.public_path.clone(),
            is_watching: false,
            rhai_template_renderer: rhai_template_renderer.clone(),
            source_filesystem: source_filesystem.clone(),
        })
        .await?
        .into();

        let prompt_controller_collection =
            build_prompt_document_controller_collection(BuildPromptControllerCollectionParams {
                asset_path_renderer: asset_path_renderer.clone(),
                content_document_linker,
                esbuild_metafile,
                rhai_template_renderer,
                source_filesystem: source_filesystem.clone(),
            })
            .await?;

        let filesystem_http_route_index =
            FilesystemHttpRouteIndex::from_filesystem(memory_filesystem.clone()).await?;

        let app_data = Data::new(AppData {});
        let search_index_reader: SearchIndexReader =
            SearchIndex::create_in_memory(content_document_sources).index()?;

        // HttpServer::new(move || {
        //     App::new()
        //         .app_data(app_data.clone())
        //         .service(
        //             Files::new(
        //                 STATIC_FILES_PUBLIC_PATH,
        //                 self.app_dir.join(STATIC_FILES_PUBLIC_PATH)
        //             ).prefer_utf8(true),
        //         )
        //         .service(McpHttpServiceFactory {
        //             mount_path: "/mcp/streamable".to_string(),
        //             prompt_controller_collection_holder: prompt_controller_collection_holder.clone(),
        //             resource_list_aggregate: resource_list_aggregate.clone(),
        //             server_info: server_info.clone(),
        //             session_manager: session_manager.clone(),
        //             tool_registry: tool_registry.clone(),
        //         })
        //         .configure(http_route::generated_pages::register)
        // })
        // .bind(self.addr)
        // .expect("Unable to bind server to address")
        // .shutdown_timeout(1)
        // .run()
        // .await?;

        Ok(())
    }
}
