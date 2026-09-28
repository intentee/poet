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
use poet_mcp::prompt_message::PromptMessage;
use poet_mdx::eval_mdx_element::eval_mdx_element;
use poet_mdx::find_text_content_in_mdast::find_text_content_in_mdast;
use poet_mdx::warn_about_unsupported_mdast_node::warn_about_unsupported_mdast_node;
use rhai_components::escape_html_attribute::escape_html_attribute;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;

use crate::markdown_code_fence::markdown_code_fence;
use crate::prompt_document_component_context::PromptDocumentComponentContext;
use crate::prompt_error::PromptError;
use crate::role_message_opening::RoleMessageOpening;

const CODE_BLOCK_FENCE_MINIMUM_LENGTH: usize = 3;
const CODE_SPAN_FENCE_MINIMUM_LENGTH: usize = 1;

fn into_blockquote(content: &str) -> String {
    content
        .lines()
        .map(|line| format!("> {line}"))
        .collect::<Vec<String>>()
        .join("\n")
}

fn render_inline_code(code: &str) -> String {
    let fence = markdown_code_fence(code, CODE_SPAN_FENCE_MINIMUM_LENGTH);
    let needs_padding = code.starts_with('`')
        || code.ends_with('`')
        || (code.starts_with(' ')
            && code.ends_with(' ')
            && code.contains(|character| character != ' '));

    if needs_padding {
        format!("{fence} {code} {fence}")
    } else {
        format!("{fence}{code}{fence}")
    }
}

fn render_title(title: Option<&str>) -> String {
    title.map_or_else(String::new, |title| {
        format!(" \"{}\"", escape_html_attribute(title))
    })
}

fn require_blank(content: String) -> Result<(), PromptError> {
    if content.trim().is_empty() {
        Ok(())
    } else {
        Err(PromptError::ContentWithoutRoleMarker { content })
    }
}

pub struct PromptDocumentEvaluator<'evaluator> {
    pub component_context: &'evaluator PromptDocumentComponentContext,
    pub rhai_template_renderer: &'evaluator RhaiTemplateRenderer,
}

impl PromptDocumentEvaluator<'_> {
    pub fn assemble_messages(&self, mdast: &Node) -> Result<Vec<PromptMessage>, PromptError> {
        mdast
            .children()
            .into_iter()
            .flatten()
            .try_for_each(|block| self.eval_block(block))
            .and_then(|()| self.component_context.flush())
            .map(|()| self.component_context.take_prompt_messages())
    }

    pub fn eval(&self, mdast: &Node) -> Result<String, PromptError> {
        match mdast {
            Node::Blockquote(Blockquote { children, .. }) => self
                .eval_children(children)
                .map(|content| into_blockquote(&content)),
            Node::Break(_) => Ok("  \n".to_owned()),
            Node::Code(Code { lang, value, .. }) => {
                let fence = markdown_code_fence(value, CODE_BLOCK_FENCE_MINIMUM_LENGTH);

                Ok(format!(
                    "{fence}{}\n{value}\n{fence}",
                    lang.as_deref().unwrap_or_default()
                ))
            }
            Node::Delete(Delete { children, .. }) => self.wrap("~~", children),
            Node::Emphasis(Emphasis { children, .. }) => self.wrap("*", children),
            Node::Heading(Heading {
                children, depth, ..
            }) => self
                .eval_children(children)
                .map(|content| format!("{} {content}", "#".repeat(usize::from(*depth)))),
            Node::Html(Html { value, .. }) | Node::Text(Text { value, .. }) => Ok(value.clone()),
            Node::Image(Image {
                alt, title, url, ..
            }) => self.eval_image(alt, title.as_deref(), url),
            Node::InlineCode(InlineCode { value, .. }) => Ok(render_inline_code(value)),
            Node::Link(Link {
                children,
                title,
                url,
                ..
            }) => self.eval_link(children, title.as_deref(), url),
            Node::List(List { children, .. }) | Node::Paragraph(Paragraph { children, .. }) => self
                .eval_children(children)
                .map(|content| format!("\n{content}\n")),
            Node::ListItem(ListItem { children, .. }) => self
                .eval_children(children)
                .map(|content| format!("- {content}")),
            Node::MdxFlowExpression(MdxFlowExpression { value, .. })
            | Node::MdxTextExpression(MdxTextExpression { value, .. }) => {
                self.eval_expression(value)
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
            }) => self.eval_children(children).and_then(|evaluated_children| {
                eval_mdx_element(
                    attributes,
                    children,
                    self.component_context,
                    evaluated_children,
                    name.as_ref(),
                    self.rhai_template_renderer,
                )
                .map_err(PromptError::EvaluateMdxElement)
            }),
            Node::Root(Root { children, .. }) | Node::Table(Table { children, .. }) => {
                self.eval_children(children)
            }
            Node::Strong(Strong { children, .. }) => self.wrap("**", children),
            Node::TableCell(TableCell { children, .. }) => self
                .eval_children(children)
                .map(|content| format!("| {content}")),
            Node::TableRow(TableRow { children, .. }) => self
                .eval_children(children)
                .map(|content| format!("{content} |")),
            Node::ThematicBreak(_) => Ok("---".to_owned()),
            Node::Toml(_) => Ok(String::new()),
            unsupported_node @ (Node::Definition(_)
            | Node::FootnoteDefinition(_)
            | Node::FootnoteReference(_)
            | Node::ImageReference(_)
            | Node::InlineMath(_)
            | Node::LinkReference(_)
            | Node::Math(_)
            | Node::MdxjsEsm(_)) => {
                warn_about_unsupported_mdast_node(unsupported_node);

                Ok(String::new())
            }
            Node::Yaml(yaml) => {
                warn!("YAML front matter is not supported, use TOML instead: {yaml:?}");

                Ok(String::new())
            }
        }
    }

    pub fn eval_children(&self, children: &[Node]) -> Result<String, PromptError> {
        children.iter().map(|child| self.eval(child)).collect()
    }

    fn append_role_message(
        &self,
        RoleMessageOpening {
            first_content,
            rest,
        }: RoleMessageOpening<'_>,
    ) -> Result<(), PromptError> {
        self.eval_children(rest).map(|rest_content| {
            self.component_context
                .append_to_message(format!("{first_content}{rest_content}").trim());
        })
    }

    fn eval_block(&self, block: &Node) -> Result<(), PromptError> {
        match block {
            Node::Paragraph(Paragraph { children, .. }) => self.eval_paragraph_block(children),
            other_block => self.eval(other_block).and_then(require_blank),
        }
    }

    fn eval_expression(&self, expression: &str) -> Result<String, PromptError> {
        self.rhai_template_renderer
            .render_expression(self.component_context.clone(), expression)
            .map(|value| value.to_string())
            .map_err(|source| PromptError::EvaluateExpression {
                expression: expression.to_owned(),
                source,
            })
    }

    fn eval_image(&self, alt: &str, title: Option<&str>, url: &str) -> Result<String, PromptError> {
        self.component_context
            .asset_manager
            .image_source(url)
            .map(|image_source| {
                format!(
                    "![{}]({}{})",
                    escape_html_attribute(alt),
                    escape_html_attribute(&image_source),
                    render_title(title)
                )
            })
            .map_err(|source| PromptError::ResolveImage {
                url: url.to_owned(),
                source,
            })
    }

    fn eval_link(
        &self,
        children: &[Node],
        title: Option<&str>,
        url: &str,
    ) -> Result<String, PromptError> {
        self.eval_children(children).and_then(|content| {
            self.component_context
                .content_document_linker
                .resolve_link(url)
                .map(|link| format!("[{content}]({link}{})", render_title(title)))
                .map_err(PromptError::ResolveLink)
        })
    }

    fn eval_paragraph_block(&self, children: &[Node]) -> Result<(), PromptError> {
        match children {
            [role_marker @ Node::Strong(_), message @ ..] => RoleMessageOpening::parse(message)
                .map_or_else(
                    || self.eval_unmarked_nodes(children),
                    |role_message_opening| {
                        self.component_context
                            .switch_role_to(&find_text_content_in_mdast(role_marker))
                            .and_then(|()| self.append_role_message(role_message_opening))
                    },
                ),
            [
                Node::MdxTextExpression(MdxTextExpression { value, .. }),
                message @ ..,
            ] => RoleMessageOpening::parse(message).map_or_else(
                || self.eval_unmarked_nodes(children),
                |role_message_opening| {
                    self.eval_expression(value)
                        .and_then(require_blank)
                        .and_then(|()| self.append_role_message(role_message_opening))
                },
            ),
            message => RoleMessageOpening::parse(message).map_or_else(
                || self.eval_unmarked_nodes(children),
                |role_message_opening| self.append_role_message(role_message_opening),
            ),
        }
    }

    fn eval_unmarked_nodes(&self, nodes: &[Node]) -> Result<(), PromptError> {
        self.eval_children(nodes).and_then(require_blank)
    }

    fn wrap(&self, marker: &str, children: &[Node]) -> Result<String, PromptError> {
        self.eval_children(children)
            .map(|content| format!("{marker}{content}{marker}"))
    }
}
