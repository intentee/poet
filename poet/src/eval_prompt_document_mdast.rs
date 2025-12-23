use anyhow::Result;
use anyhow::anyhow;
use log::warn;
use markdown::mdast::Blockquote;
use markdown::mdast::Code;
use markdown::mdast::Delete;
use markdown::mdast::Emphasis;
use markdown::mdast::Heading;
use markdown::mdast::Html;
use markdown::mdast::Image;
use markdown::mdast::InlineCode;
use markdown::mdast::Link;
use markdown::mdast::List;
use markdown::mdast::ListItem;
use markdown::mdast::MdxFlowExpression;
use markdown::mdast::MdxJsxFlowElement;
use markdown::mdast::MdxJsxTextElement;
use markdown::mdast::MdxTextExpression;
use markdown::mdast::Node;
use markdown::mdast::Paragraph;
use markdown::mdast::Root;
use markdown::mdast::Strong;
use markdown::mdast::Table;
use markdown::mdast::TableCell;
use markdown::mdast::TableRow;
use markdown::mdast::Text;
use markdown::mdast::ThematicBreak;
use rhai_components::escape_html::escape_html;
use rhai_components::escape_html_attribute::escape_html_attribute;

use crate::eval_mdx_element::eval_mdx_element;
use crate::eval_prompt_document_mdast_params::EvalPromptDocumentMdastParams;
use crate::is_external_link::is_external_link;
use crate::prompt_document_component_context::PromptDocumentComponentContext;

fn into_blockquote(input: String) -> String {
    input
        .lines()
        .map(|line| format!("> {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn trim_chunk(chunk: String) -> Result<String> {
    if chunk.is_empty() {
        return Ok(chunk);
    }

    Ok(chunk
        .trim()
        .strip_prefix(':')
        .ok_or_else(|| anyhow!("Unable to strip chunk prefix from '{chunk}'"))?
        .trim_start()
        .to_string())
}

pub fn eval_prompt_document_children(
    children: &Vec<Node>,
    params: EvalPromptDocumentMdastParams,
    prompt_document_component_context: &mut PromptDocumentComponentContext,
) -> Result<String> {
    let mut content = String::new();
    let mut is_first_child = true;

    for child in children {
        content.push_str(&eval_prompt_document_mdast(
            params.child(child, is_first_child),
            prompt_document_component_context,
        )?);

        is_first_child = false;
    }

    Ok(content)
}

/// Converts Markdown syntax into tidied up Markdown with resolved image paths,
/// references, and such
pub fn eval_prompt_document_mdast(
    params @ EvalPromptDocumentMdastParams {
        mdast,
        is_directly_in_root,
        is_first_child,
        is_in_top_paragraph,
        rhai_template_renderer,
    }: EvalPromptDocumentMdastParams,
    prompt_document_component_context: &mut PromptDocumentComponentContext,
) -> Result<String> {
    let mut result = String::new();

    match mdast {
        Node::Blockquote(Blockquote { children, .. }) => {
            result.push_str(&into_blockquote(eval_prompt_document_children(
                children,
                params.regular_element(),
                prompt_document_component_context,
            )?));
        }
        Node::Break(_) => {
            result.push_str("  \n");
        }
        Node::Code(Code { lang, value, .. }) => {
            result.push_str(&format!("```{}\n", lang.clone().unwrap_or("".to_string())));
            result.push_str(&escape_html(value));
            result.push_str("\n```");
        }
        Node::Definition(node) => {
            warn!("Definitions are not supported: {node:?}");
        }
        Node::Delete(Delete { children, .. }) => {
            result.push_str("~~");
            result.push_str(&eval_prompt_document_children(
                children,
                params.regular_element(),
                prompt_document_component_context,
            )?);
            result.push_str("~~");
        }
        Node::Emphasis(Emphasis { children, .. }) => {
            result.push('*');
            result.push_str(&eval_prompt_document_children(
                children,
                params.regular_element(),
                prompt_document_component_context,
            )?);
            result.push('*');
        }
        Node::FootnoteDefinition(node) => {
            warn!("Footnote definitions are not supported: {node:?}");
        }
        Node::FootnoteReference(node) => {
            warn!("Footnote references are not supported: {node:?}");
        }
        Node::Heading(Heading {
            children, depth, ..
        }) => {
            result.push_str(&("#".repeat(*depth as usize)));
            result.push_str(&eval_prompt_document_children(
                children,
                params.regular_element(),
                prompt_document_component_context,
            )?);
        }
        Node::Html(Html { value, .. }) => {
            result.push_str(value);
        }
        Node::Image(Image {
            alt, url, title, ..
        }) => {
            result.push_str(&format!("![{}](", escape_html_attribute(alt)));

            let src = if is_external_link(url) {
                url
            } else {
                &match prompt_document_component_context.asset_manager.file(url) {
                    Ok(src) => src,
                    Err(err) => return Err(anyhow!(err)),
                }
            };

            result.push_str(&escape_html_attribute(src));

            if let Some(title) = title {
                result.push_str(&format!(" \"{}\"", escape_html_attribute(title)));
            }

            result.push(')');
        }
        Node::ImageReference(node) => {
            warn!("Image references are not supported: {node:?}");
        }
        Node::InlineCode(InlineCode { value, .. }) => {
            result.push_str(&format!("`{}`", escape_html_attribute(value)));
        }
        Node::InlineMath(node) => {
            warn!("Inline math expressions are not supported: {node:?}");
        }
        Node::Link(Link {
            children,
            title,
            url,
            ..
        }) => {
            result.push_str(&format!(
                "[{}]",
                eval_prompt_document_children(
                    children,
                    params.regular_element(),
                    prompt_document_component_context
                )?
            ));

            let link = if is_external_link(url) {
                url.clone()
            } else {
                match prompt_document_component_context
                    .content_document_linker
                    .link_to(url)
                {
                    Ok(link) => link,
                    Err(err) => return Err(anyhow!(err)),
                }
            };

            result.push_str(&format!("({link}"));

            if let Some(title) = title {
                result.push_str(&format!(" \"{}\"", title));
            }

            result.push(')');
        }
        Node::LinkReference(node) => {
            warn!("Link references are not supported: {node:?}");
        }
        Node::List(List { children, .. }) => {
            result.push('\n');

            result.push_str(&eval_prompt_document_children(
                children,
                params.regular_element(),
                prompt_document_component_context,
            )?);

            result.push('\n');
        }
        Node::ListItem(ListItem { children, .. }) => {
            result.push_str("- ");
            result.push_str(&eval_prompt_document_children(
                children,
                params.regular_element(),
                prompt_document_component_context,
            )?);
        }
        Node::Math(node) => {
            warn!("Math expressions are not supported: {node:?}");
        }
        Node::MdxjsEsm(node) => {
            warn!("MDX ESM expressions are not supported: {node:?}");
        }
        Node::MdxFlowExpression(MdxFlowExpression { value, .. })
        | Node::MdxTextExpression(MdxTextExpression { value, .. }) => {
            result.push_str(
                &rhai_template_renderer
                    .render_expression(prompt_document_component_context.clone(), value)?
                    .to_string(),
            );
        }
        Node::MdxJsxFlowElement(MdxJsxFlowElement {
            attributes,
            children,
            name,
            ..
        })
        | Node::MdxJsxTextElement(MdxJsxTextElement {
            attributes,
            children,
            name,
            ..
        }) => {
            let evaluated_children = eval_prompt_document_children(
                children,
                params.regular_element(),
                prompt_document_component_context,
            )?;

            result.push_str(&eval_mdx_element(
                attributes,
                children,
                prompt_document_component_context,
                evaluated_children,
                name,
                rhai_template_renderer,
            )?);
        }
        Node::Paragraph(Paragraph { children, .. }) => {
            result.push('\n');
            result.push_str(&eval_prompt_document_children(
                children,
                params.paragraph(),
                prompt_document_component_context,
            )?);
            result.push('\n');
        }
        Node::Root(Root { children, .. }) => {
            result.push_str(&eval_prompt_document_children(
                children,
                params.directly_in_root(),
                prompt_document_component_context,
            )?);

            prompt_document_component_context.flush()?;
        }
        Node::Strong(Strong { children, .. }) => {
            let potential_role_name: &str = &eval_prompt_document_children(
                children,
                params.regular_element(),
                prompt_document_component_context,
            )?;

            if is_first_child && is_in_top_paragraph {
                prompt_document_component_context
                    .switch_role_to(potential_role_name.try_into()?)?;
            } else {
                result.push_str("**");
                result.push_str(potential_role_name);
                result.push_str("**");
            }
        }
        Node::Table(Table { children, .. }) => {
            result.push_str(&eval_prompt_document_children(
                children,
                params.regular_element(),
                prompt_document_component_context,
            )?);
        }
        Node::TableCell(TableCell { children, .. }) => {
            result.push_str("| ");
            result.push_str(&eval_prompt_document_children(
                children,
                params.regular_element(),
                prompt_document_component_context,
            )?);
        }
        Node::TableRow(TableRow { children, .. }) => {
            result.push_str(&eval_prompt_document_children(
                children,
                params.regular_element(),
                prompt_document_component_context,
            )?);
            result.push_str(" |");
        }
        Node::Text(Text { value, .. }) => {
            result.push_str(value);
        }
        Node::ThematicBreak(ThematicBreak { .. }) => {
            result.push_str("---");
        }
        Node::Toml(_) => {
            // ignore frontmatter during this pass
        }
        Node::Yaml(node) => {
            warn!("YAML front-matter is not supported, use TOML instead: {node:?}");
        }
    }

    if is_directly_in_root {
        prompt_document_component_context.append_to_message(trim_chunk(result.clone())?)?;
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_blockquotes() {
        assert_eq!(
            into_blockquote("foo\nbar\nbaz".to_string()),
            "> foo\n> bar\n> baz".to_string()
        );
    }

    #[test]
    fn test_chunk_trim() -> Result<()> {
        assert_eq!(
            trim_chunk(
                r#"
                : foo bar
            "#
                .to_string()
            )?,
            "foo bar".to_string(),
        );

        Ok(())
    }

    #[test]
    fn test_chunk_trim_empty() -> Result<()> {
        assert_eq!(trim_chunk("".to_string())?, "".to_string(),);

        Ok(())
    }
}
