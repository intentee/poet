use anyhow::Result;
use anyhow::anyhow;

use crate::filesystem::file_entry::FileEntry;
use crate::find_front_matter_in_mdast::find_front_matter_in_mdast;
use crate::prompt_document_front_matter::PromptDocumentFrontMatter;
use crate::string_to_mdast::string_to_mdast;

pub async fn build_prompt_controller(file: FileEntry) -> Result<()> {
    let mdast = string_to_mdast(&file.contents)?;
    let front_matter: PromptDocumentFrontMatter = find_front_matter_in_mdast(&mdast)?
        .ok_or_else(|| anyhow!("No front matter found in file: {:?}", file.relative_path))?;

    Ok(())
}
