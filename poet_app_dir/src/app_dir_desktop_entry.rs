use std::fmt;
use std::path::Path;

use freedesktop_entry_parser::Entry;
use freedesktop_entry_parser::Section;
use indoc::formatdoc;
use poet_filesystem::filesystem::Filesystem;

use crate::app_dir_error::AppDirError;
use crate::app_dir_name::AppDirName;

const DESKTOP_ENTRY_SECTION: &str = "Desktop Entry";
const NAME_KEY: &str = "Name";
const POET_VERSION_KEY: &str = "X-PoetVersion";
const SITE_VERSION_KEY: &str = "X-SiteVersion";
const TITLE_KEY: &str = "X-ImplementationTitle";

fn single_attribute(section: &Section, key: &str) -> Result<String, AppDirError> {
    match section.attr(key) {
        [value] => Ok(value.clone()),
        [] => Err(AppDirError::DesktopEntryAttributeMissing {
            key: key.to_owned(),
        }),
        [_, _, ..] => Err(AppDirError::AmbiguousDesktopEntryAttribute {
            key: key.to_owned(),
        }),
    }
}

pub struct AppDirDesktopEntry {
    pub name: AppDirName,
    pub poet_version: String,
    pub site_version: String,
    pub title: String,
}

impl AppDirDesktopEntry {
    pub fn parse(input: &str) -> Result<Self, AppDirError> {
        let entry = Entry::parse(input).map_err(AppDirError::ParseDesktopEntry)?;
        let section = entry
            .section(DESKTOP_ENTRY_SECTION)
            .ok_or(AppDirError::DesktopEntrySectionMissing)?;

        Ok(Self {
            name: AppDirName::parse(&single_attribute(section, NAME_KEY)?)?,
            poet_version: single_attribute(section, POET_VERSION_KEY)?,
            site_version: single_attribute(section, SITE_VERSION_KEY)?,
            title: single_attribute(section, TITLE_KEY)?,
        })
    }

    pub async fn read_from<TFilesystem: Filesystem>(
        filesystem: &TFilesystem,
        app_dir_name: &AppDirName,
    ) -> Result<Self, AppDirError> {
        filesystem
            .read_file_contents_string(Path::new(&app_dir_name.desktop_file_name()))
            .await
            .map_err(AppDirError::ReadDesktopEntry)
            .and_then(|contents| Self::parse(&contents))
    }
}

impl fmt::Display for AppDirDesktopEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}",
            formatdoc! {
                "
                    [{DESKTOP_ENTRY_SECTION}]
                    Categories=System;
                    Icon={name}
                    {NAME_KEY}={name}
                    Terminal=true
                    Type=Application
                    {TITLE_KEY}={title}
                    {POET_VERSION_KEY}={poet_version}
                    {SITE_VERSION_KEY}={site_version}
                ",
                name = self.name,
                poet_version = self.poet_version,
                site_version = self.site_version,
                title = self.title,
            }
        )
    }
}
