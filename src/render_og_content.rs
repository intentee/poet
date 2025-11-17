use anyhow::{Error, Result};
use indoc::formatdoc;
use url::Url;

use crate::{asset_manager::AssetManager, content_document_reference::ContentDocumentReference};

pub fn render_og_content(
    reference: ContentDocumentReference,
    asset_manager: AssetManager,
) -> Result<String, Error> {
    let front_matter = reference.clone().front_matter;

    if !front_matter.image.is_empty() {
        let canonical_url = reference
            .clone()
            .canonical_link()
            .map_err(|e| anyhow::anyhow!(e))?;
        let image = match Url::parse(&front_matter.image) {
            Ok(image) => image.to_string(),
            Err(_) => asset_manager
                .file(&front_matter.image)
                .map_err(|e| anyhow::anyhow!(e))?,
        };

        return Ok(formatdoc! {r#"
            <meta property="og:type" content="website" />
            <meta property="og:image:alt" content="{}" />
            <meta property="og:image" content="{}" />
            
            <meta property="og:title" content="{}" />
            <meta property="og:url" content="{}" />
            <meta property="og:description" content="{}" />
        "#, front_matter.title.clone(), image, front_matter.title, canonical_url, front_matter.description});
    }

    Ok("".to_string())
}
