use std::fs::create_dir_all;
use std::fs::write;
use std::io;

use tempfile::TempDir;
use tempfile::tempdir;

const ESBUILD_METAFILE: &str = r#"{
  "outputs": {
    "static/logo_ABCDEF12.png": {
      "imports": [],
      "inputs": { "logo.png": {} }
    }
  }
}"#;

pub struct FixtureSite {
    pub output_directory: TempDir,
    pub source_directory: TempDir,
}

impl FixtureSite {
    pub fn create() -> io::Result<Self> {
        let source_directory = tempdir()?;

        create_dir_all(source_directory.path().join("authors"))?;
        create_dir_all(source_directory.path().join("content"))?;
        create_dir_all(source_directory.path().join("prompts"))?;
        create_dir_all(source_directory.path().join("shortcodes"))?;
        create_dir_all(source_directory.path().join("static"))?;
        write(
            source_directory.path().join("authors/ada.toml"),
            "name = \"Ada\"\n",
        )?;
        write(
            source_directory.path().join("content/index.md"),
            "+++\ntitle = \"Home\"\n+++\n",
        )?;
        write(
            source_directory.path().join("prompts/greet.md"),
            "**user**: hi\n",
        )?;
        write(
            source_directory.path().join("shortcodes/Layout.rhai"),
            "fn template() {}\n",
        )?;
        write(
            source_directory.path().join("static/logo_ABCDEF12.png"),
            "png",
        )?;
        write(
            source_directory.path().join("esbuild-meta.json"),
            ESBUILD_METAFILE,
        )?;

        Ok(Self {
            output_directory: tempdir()?,
            source_directory,
        })
    }
}
