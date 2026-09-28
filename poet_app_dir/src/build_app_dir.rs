use std::env::consts::ARCH;
use std::path::Path;
use std::path::PathBuf;

use indoc::formatdoc;
use log::info;
use poet_assets::copy_esbuild_metafile_assets_to::copy_esbuild_metafile_assets_to;
use poet_assets::esbuild_metafile_path::ESBUILD_METAFILE_PATH;
use poet_assets::read_esbuild_metafile_or_default::read_esbuild_metafile_or_default;
use poet_content::authors_source_directory::AUTHORS_SOURCE_DIRECTORY;
use poet_content::content_source_directory::CONTENT_SOURCE_DIRECTORY;
use poet_filesystem::filesystem::Filesystem as _;
use poet_filesystem::storage::Storage;
use poet_mdx::shortcodes_source_directory::SHORTCODES_SOURCE_DIRECTORY;
use poet_prompt::prompts_source_directory::PROMPTS_SOURCE_DIRECTORY;

use crate::app_dir_error::AppDirError;
use crate::app_dir_file::AppDirFile;
use crate::app_dir_name::AppDirName;
use crate::build_app_dir_params::BuildAppDirParams;

const APP_RUN_FILE_NAME: &str = "AppRun";
const EXECUTABLE_PERMISSIONS_MODE: u32 = 0o755;
const REGULAR_FILE_PERMISSIONS_MODE: u32 = 0o644;
const ICON: &str = r#"<svg viewBox="0 0 10 10" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect width="10" height="10" fill="black"/>
</svg>"#;

fn render_app_run_script(app_dir_name: &AppDirName) -> String {
    formatdoc! {
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

            exec $APPDIR/poet serve $APPDIR --addr "$ADDR" --app-name "{app_dir_name}" --public-path "$PUBLIC_PATH"
        "#,
    }
}

pub async fn build_app_dir(
    BuildAppDirParams {
        desktop_entry,
        output_directory,
        source_directory,
    }: BuildAppDirParams<'_>,
) -> Result<PathBuf, AppDirError> {
    let app_dir_path = output_directory.join(desktop_entry.name.app_dir_directory_name());
    let app_dir_filesystem = Storage {
        base_directory: app_dir_path.clone(),
    };
    let source_filesystem = Storage {
        base_directory: source_directory.to_path_buf(),
    };

    info!("Copying project files to AppDir...");

    for project_source_directory in [
        AUTHORS_SOURCE_DIRECTORY,
        CONTENT_SOURCE_DIRECTORY,
        PROMPTS_SOURCE_DIRECTORY,
        SHORTCODES_SOURCE_DIRECTORY,
    ] {
        app_dir_filesystem
            .copy_source_files_from(&source_filesystem, &project_source_directory)
            .await
            .map_err(AppDirError::CopyProjectFiles)?;
    }

    app_dir_filesystem
        .copy_file_from(&source_filesystem, Path::new(ESBUILD_METAFILE_PATH))
        .await
        .map_err(AppDirError::CopyProjectFiles)?;

    info!("Copying assets to AppDir...");

    let esbuild_metafile = read_esbuild_metafile_or_default(&source_filesystem)
        .await
        .map_err(AppDirError::ReadEsbuildMetafile)?;

    copy_esbuild_metafile_assets_to(&esbuild_metafile, source_directory, &app_dir_path)
        .await
        .map_err(AppDirError::CopyAssets)?;

    info!("Creating AppDir-specific metafiles...");

    for app_dir_file in [
        AppDirFile {
            contents: render_app_run_script(&desktop_entry.name),
            path: app_dir_path.join(APP_RUN_FILE_NAME),
            permissions_mode: EXECUTABLE_PERMISSIONS_MODE,
        },
        AppDirFile {
            contents: desktop_entry.to_string(),
            path: app_dir_path.join(desktop_entry.name.desktop_file_name()),
            permissions_mode: REGULAR_FILE_PERMISSIONS_MODE,
        },
        AppDirFile {
            contents: ICON.to_owned(),
            path: app_dir_path.join(desktop_entry.name.icon_file_name()),
            permissions_mode: REGULAR_FILE_PERMISSIONS_MODE,
        },
    ] {
        app_dir_file.write().await?;
    }

    let app_dir_display = app_dir_path.display();

    info!(
        "AppDir is ready. You can now run `ARCH={ARCH} appimagetool {app_dir_display}` to finish the process (you need to have AppImageKit installed)"
    );

    Ok(app_dir_path)
}
