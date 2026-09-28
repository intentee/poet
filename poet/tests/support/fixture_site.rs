use std::fs::create_dir_all;
use std::fs::write;
use std::io;
use std::path::Path;

use poet_content_tests::layout_plain_shortcode::LAYOUT_PLAIN_SHORTCODE;
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
    pub directory: TempDir,
}

impl FixtureSite {
    pub fn create() -> io::Result<Self> {
        let fixture_site = Self {
            directory: tempdir()?,
        };

        fixture_site.write("authors/ada.toml", "name = \"Ada\"\n")?;
        fixture_site.write(
            "content/index.md",
            "+++\nauthors = [\"ada\"]\ndescription = \"Home\"\nlayout = \"LayoutPlain\"\ntitle = \"Home\"\n+++\n\nHome body.\n",
        )?;
        fixture_site.write(
            "content/docs/page.md",
            "+++\ndescription = \"Page\"\nlayout = \"LayoutPlain\"\ntitle = \"Page\"\n+++\n\nPage body.\n",
        )?;
        fixture_site.write(
            "prompts/greet.md",
            "+++\narguments = {}\ndescription = \"Greets\"\ntitle = \"Greet\"\n+++\n\n**user**: Hello!\n",
        )?;
        fixture_site.write("shortcodes/LayoutPlain.rhai", LAYOUT_PLAIN_SHORTCODE)?;
        fixture_site.write("esbuild-meta.json", ESBUILD_METAFILE)?;
        fixture_site.write("static/logo_ABCDEF12.png", "png")?;
        fixture_site.write(
            "fixture.desktop",
            "[Desktop Entry]\nName=fixture\nX-PoetVersion=0.8.0\nX-SiteVersion=1.0.0\nX-ImplementationTitle=Fixture\n",
        )?;

        Ok(fixture_site)
    }

    pub fn path(&self) -> &Path {
        self.directory.path()
    }

    pub fn path_string(&self) -> String {
        self.directory.path().display().to_string()
    }

    pub fn write(&self, relative_path: &str, contents: &str) -> io::Result<()> {
        let path = self.directory.path().join(relative_path);

        if let Some(parent_directory) = path.parent() {
            create_dir_all(parent_directory)?;
        }

        write(path, contents)
    }
}
