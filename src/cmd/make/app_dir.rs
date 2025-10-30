use std::env::consts::ARCH;
use std::os::unix::fs::PermissionsExt as _;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use indoc::formatdoc;
use log::info;
use tokio::fs;

use crate::app_dir_desktop_entry::AppDirDesktopEntry;
use crate::assert_valid_desktop_entry_string::assert_valid_desktop_entry_string;
use crate::cmd::builds_project::BuildsProject;
use crate::cmd::handler::Handler;
use crate::cmd::value_parser::validate_is_directory;
use crate::cmd::value_parser::validate_is_directory_or_create;
use crate::copy_esbuild_metafile_assets_to::copy_esbuild_metafile_assets_to;
use crate::filesystem::Filesystem;
use crate::filesystem::storage::Storage;
use crate::read_esbuild_metafile_or_default::read_esbuild_metafile_or_default;

const ICON: &str = r#"<svg viewBox="0 0 10 10" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect width="10" height="10" fill="black"/>
</svg>"#;

#[derive(Parser)]
pub struct AppDir {
    #[arg(long)]
    name: String,

    #[arg(long, value_parser = validate_is_directory_or_create)]
    output_directory: PathBuf,

    #[arg(value_parser = validate_is_directory)]
    source_directory: PathBuf,

    #[arg(long)]
    title: String,

    #[arg(long)]
    version: String,
}

impl AppDir {
    fn app_dir_path(&self) -> PathBuf {
        self.output_directory.join(format!("{}.AppDir", self.name))
    }

    fn render_app_run_file(&self) -> Result<String> {
        Ok(formatdoc! {
            r#"
                #!/usr/bin/env sh

                ADDR=""
                PUBLIC_PATH=""

                while [ $# -gt 0 ]; do
                    case $1 in
                        --addr)
                            ADDR="$2"
                            shift 2
                            ;;
                        --public-path)
                            PUBLIC_PATH="$2"
                            shift 2
                            ;;
                        *)
                            echo "Unknown argument: $1"
                            echo "Usage: $0 --addr ADDRESS --public-path PATH"
                            exit 1
                            ;;
                    esac
                done

                if [ -z "$ADDR" ]; then
                    echo "Error: --addr is required"
                    echo "Usage: $0 --addr ADDRESS --public-path PATH"
                    exit 1
                fi

                if [ -z "$PUBLIC_PATH" ]; then
                    echo "Error: --public-path is required"
                    echo "Usage: $0 --addr ADDRESS --public-path PATH"
                    exit 1
                fi

                exec $APPDIR/poet serve $APPDIR --addr "$ADDR" --app-name "{name}" --public-path "$PUBLIC_PATH"
            "#,
            name = self.name,
        })
    }

    fn render_desktop_file(&self) -> Result<String> {
        Ok(AppDirDesktopEntry {
            name: assert_valid_desktop_entry_string(&self.name)?,
            poet_version: env!("CARGO_PKG_VERSION").to_string(),
            site_version: assert_valid_desktop_entry_string(&self.version)?,
            title: assert_valid_desktop_entry_string(&self.title)?,
        }
        .to_string())
    }
}

impl BuildsProject for AppDir {
    fn source_directory(&self) -> PathBuf {
        self.source_directory.clone()
    }
}

#[async_trait(?Send)]
impl Handler for AppDir {
    async fn handle(&self) -> Result<()> {
        let app_dir_path = self.app_dir_path();
        let source_filesystem = self.source_filesystem();
        let name_lowercase = self.name.to_lowercase();

        let app_dir_filesystem = Arc::new(Storage {
            base_directory: app_dir_path.clone(),
        });

        info!("Copying project files to AppDir...");

        app_dir_filesystem
            .copy_project_files_from(source_filesystem.clone())
            .await?;
        app_dir_filesystem
            .copy_file_from(
                source_filesystem.clone(),
                &PathBuf::from("esbuild-meta.json"),
            )
            .await?;

        info!("Copying assets to AppDir...");

        let esbuild_metafile = read_esbuild_metafile_or_default(source_filesystem.clone()).await?;

        copy_esbuild_metafile_assets_to(esbuild_metafile, &app_dir_path).await?;

        info!("Creating AppDir-specific metafiles...");

        fs::create_dir_all(&app_dir_path).await?;

        let apprun_path = app_dir_path.join("AppRun");

        fs::write(&apprun_path, self.render_app_run_file()?).await?;

        let mut perms = fs::metadata(&apprun_path).await?.permissions();

        perms.set_mode(0o755);

        fs::set_permissions(&apprun_path, perms).await?;

        fs::write(
            app_dir_path.join(format!("{name_lowercase}.desktop")),
            self.render_desktop_file()?,
        )
        .await?;
        fs::write(app_dir_path.join(format!("{name_lowercase}.svg")), ICON).await?;

        info!(
            "AppDir is ready. You can now run `ARCH={} appimagetool {}` to finish the process (you need to have AppImageKit installed)",
            ARCH,
            app_dir_path.display()
        );

        Ok(())
    }
}
