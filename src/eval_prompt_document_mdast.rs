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

use crate::escape_html::escape_html;
use crate::escape_html_attribute::escape_html_attribute;
use crate::eval_mdx_element::eval_mdx_element;
use crate::is_external_link::is_external_link;
use crate::prompt_document_component_context::PromptDocumentComponentContext;
use crate::rhai_template_renderer::RhaiTemplateRenderer;

fn into_blockquote(input: String) -> String {
    input
        .lines()
        .map(|line| format!("> {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn eval_prompt_document_children(
    children: &Vec<Node>,
    component_context: &PromptDocumentComponentContext,
    rhai_template_renderer: &RhaiTemplateRenderer,
) -> Result<String> {
    let mut content = String::new();

    for child in children {
        content.push_str(&eval_prompt_document_mdast(
            child,
            component_context,
            rhai_template_renderer,
        )?);
    }

    Ok(content)
}

/// Converts Markdown syntax into tidied up Markdown with resolved image paths,
/// references, and such
pub fn eval_prompt_document_mdast(
    mdast: &Node,
    component_context: &PromptDocumentComponentContext,
    rhai_template_renderer: &RhaiTemplateRenderer,
) -> Result<String> {
    let mut result = String::new();

    match mdast {
        Node::Blockquote(Blockquote { children, .. }) => {
            result.push_str(&into_blockquote(eval_prompt_document_children(
                children,
                component_context,
                rhai_template_renderer,
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
                component_context,
                rhai_template_renderer,
            )?);
            result.push_str("~~");
        }
        Node::Emphasis(Emphasis { children, .. }) => {
            result.push('*');
            result.push_str(&eval_prompt_document_children(
                children,
                component_context,
                rhai_template_renderer,
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
                component_context,
                rhai_template_renderer,
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
                &match component_context.asset_manager.file(url) {
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
                eval_prompt_document_children(children, component_context, rhai_template_renderer,)?
            ));

            let link = if is_external_link(url) {
                url.clone()
            } else {
                match component_context.content_document_linker.link_to(url) {
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
                component_context,
                rhai_template_renderer,
            )?);

            result.push('\n');
        }
        Node::ListItem(ListItem { children, .. }) => {
            result.push_str("- ");
            result.push_str(&eval_prompt_document_children(
                children,
                component_context,
                rhai_template_renderer,
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
                    .render_expression(component_context.clone(), value)?
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
            result.push_str(&eval_mdx_element(
                attributes,
                children,
                component_context,
                eval_prompt_document_children(children, component_context, rhai_template_renderer)?,
                name,
                rhai_template_renderer,
            )?);
        }
        Node::Paragraph(Paragraph { children, .. }) => {
            result.push('\n');
            result.push_str(&eval_prompt_document_children(
                children,
                component_context,
                rhai_template_renderer,
            )?);
            result.push('\n');
        }
        Node::Root(Root { children, .. }) => {
            result.push_str(&eval_prompt_document_children(
                children,
                component_context,
                rhai_template_renderer,
            )?);
        }
        Node::Strong(Strong { children, .. }) => {
            result.push_str("**");
            result.push_str(&eval_prompt_document_children(
                children,
                component_context,
                rhai_template_renderer,
            )?);
            result.push_str("**");
        }
        Node::Table(Table { children, .. }) => {
            result.push_str(&eval_prompt_document_children(
                children,
                component_context,
                rhai_template_renderer,
            )?);
        }
        Node::TableCell(TableCell { children, .. }) => {
            result.push_str("| ");
            result.push_str(&eval_prompt_document_children(
                children,
                component_context,
                rhai_template_renderer,
            )?);
        }
        Node::TableRow(TableRow { children, .. }) => {
            result.push_str(&eval_prompt_document_children(
                children,
                component_context,
                rhai_template_renderer,
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
}
