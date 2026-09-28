use poet_content::content_document_reference::ContentDocumentReference;

use crate::fixture_reference::fixture_reference;

pub fn fixture_docs_references() -> Result<Vec<ContentDocumentReference>, toml::de::Error> {
    Ok(vec![
        fixture_reference(
            "docs/index",
            "description = \"d\"\nid = \"docs-home\"\nlayout = \"L\"\ntitle = \"Docs\"\n\n[[collection]]\nname = \"docs\"",
        )?,
        fixture_reference(
            "docs/second",
            "description = \"d\"\nlayout = \"L\"\ntitle = \"Second\"\n\n[[collection]]\nafter = \"docs/index\"\nname = \"docs\"",
        )?,
        fixture_reference(
            "docs/hidden",
            "description = \"d\"\nlayout = \"L\"\nrender = false\ntitle = \"Hidden\"\n\n[[collection]]\nafter = \"docs/second\"\nname = \"docs\"",
        )?,
        fixture_reference(
            "docs/last",
            "description = \"d\"\nlayout = \"L\"\ntitle = \"Last\"\n\n[[collection]]\nafter = \"docs/hidden\"\nname = \"docs\"",
        )?,
        fixture_reference(
            "docs/child",
            "description = \"d\"\nlayout = \"L\"\ntitle = \"Child\"\n\n[[collection]]\nname = \"docs\"\nparent = \"docs/index\"",
        )?,
        fixture_reference(
            "multi/declared",
            "description = \"d\"\nlayout = \"L\"\nprimary_collection = \"guides\"\ntitle = \"Declared\"\n\n[[collection]]\nname = \"guides\"\n\n[[collection]]\nname = \"tutorials\"",
        )?,
        fixture_reference(
            "multi/undeclared",
            "description = \"d\"\nlayout = \"L\"\ntitle = \"Undeclared\"\n\n[[collection]]\nname = \"guides\"\n\n[[collection]]\nname = \"tutorials\"",
        )?,
        fixture_reference(
            "about",
            "description = \"d\"\nlayout = \"L\"\ntitle = \"About\"",
        )?,
    ])
}
