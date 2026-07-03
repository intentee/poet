use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use anyhow::Result;
use anyhow::anyhow;
use freedesktop_entry_parser::Entry;
use freedesktop_entry_parser::Section;
use indoc::formatdoc;

struct AppDirDesktopEntryStub {
    pub entry: Entry,
}

impl AppDirDesktopEntryStub {
    pub fn get_attr_single(&self, key: &str) -> Result<String> {
        match self.get_desktop_entry_section()?.attr(key) {
            [title] => Ok(title.to_string()),
            [_title, ..] => Err(anyhow!("Ambiguous {key} attribute (more than one value)")),
            [] => Err(anyhow!("{key} field is missing")),
        }
    }

    fn get_desktop_entry_section(&self) -> Result<Section> {
        Ok(self
            .entry
            .section("Desktop Entry")
            .ok_or_else(|| anyhow!("Desktop entry is missing it's primary section"))?
            .clone())
    }
}

pub struct AppDirDesktopEntry {
    pub name: String,
    pub poet_version: String,
    pub site_version: String,
    pub title: String,
}

impl AppDirDesktopEntry {
    pub fn parse(input: &str) -> Result<Self> {
        let stub = AppDirDesktopEntryStub {
            entry: Entry::parse(input)?,
        };

        Ok(AppDirDesktopEntry {
            name: stub.get_attr_single("Name")?,
            poet_version: stub.get_attr_single("X-PoetVersion")?,
            site_version: stub.get_attr_single("X-SiteVersion")?,
            title: stub.get_attr_single("X-ImplementationTitle")?,
        })
    }
}

impl Display for AppDirDesktopEntry {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}",
            formatdoc! {
                r#"
                    [Desktop Entry]
                    Categories=System;
                    Icon={name}
                    Name={name}
                    Terminal=true
                    Type=Application
                    X-ImplementationTitle={title}
                    X-PoetVersion={poet_version}
                    X-SiteVersion={site_version}
                "#,
                name = self.name,
                poet_version = self.poet_version,
                site_version = self.site_version,
                title = self.title,
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn parses_all_fields_from_desktop_entry() -> Result<()> {
        let entry = AppDirDesktopEntry::parse(indoc! {r#"
            [Desktop Entry]
            Name=mysite
            X-PoetVersion=0.6.2
            X-SiteVersion=1.2.3
            X-ImplementationTitle=My Site
        "#})?;

        assert_eq!(entry.name, "mysite");
        assert_eq!(entry.poet_version, "0.6.2");
        assert_eq!(entry.site_version, "1.2.3");
        assert_eq!(entry.title, "My Site");

        Ok(())
    }

    #[test]
    fn errors_when_required_field_is_missing() {
        assert!(
            AppDirDesktopEntry::parse(indoc! {r#"
                [Desktop Entry]
                Name=mysite
                X-SiteVersion=1.2.3
                X-ImplementationTitle=My Site
            "#})
            .is_err()
        );
    }

    #[test]
    fn errors_when_field_is_defined_more_than_once() {
        assert!(
            AppDirDesktopEntry::parse(indoc! {r#"
                [Desktop Entry]
                Name=mysite
                Name=othersite
                X-PoetVersion=0.6.2
                X-SiteVersion=1.2.3
                X-ImplementationTitle=My Site
            "#})
            .is_err()
        );
    }

    #[test]
    fn errors_when_primary_section_is_missing() {
        assert!(
            AppDirDesktopEntry::parse(indoc! {r#"
                [Other Section]
                Name=mysite
            "#})
            .is_err()
        );
    }

    #[test]
    fn renders_desktop_entry_with_all_fields() {
        let rendered = AppDirDesktopEntry {
            name: "mysite".to_string(),
            poet_version: "0.6.2".to_string(),
            site_version: "1.2.3".to_string(),
            title: "My Site".to_string(),
        }
        .to_string();

        assert!(rendered.contains("Name=mysite"));
        assert!(rendered.contains("X-PoetVersion=0.6.2"));
        assert!(rendered.contains("X-SiteVersion=1.2.3"));
        assert!(rendered.contains("X-ImplementationTitle=My Site"));
    }
}
