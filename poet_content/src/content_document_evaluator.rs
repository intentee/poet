use log::warn;
use markdown::mdast::Blockquote;
use markdown::mdast::Code;
use markdown::mdast::Delete;
use markdown::mdast::Emphasis;
use markdown::mdast::FootnoteReference;
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
use poet_mdx::eval_mdx_element::eval_mdx_element;
use poet_mdx::mdast_container_children::mdast_container_children;
use poet_mdx::warn_about_unsupported_mdast_node::warn_about_unsupported_mdast_node;
use rhai_components::escape_html::escape_html;
use rhai_components::escape_html_attribute::escape_html_attribute;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;
use syntect::html::ClassStyle;
use syntect::html::ClassedHTMLGenerator;
use syntect::parsing::SyntaxReference;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use crate::content_document_component_context::ContentDocumentComponentContext;
use crate::content_error::ContentError;
use crate::mdast_children_to_heading_id::mdast_children_to_heading_id;
use crate::metadata_line_item::MetadataLineItem;
use crate::parse_markdown_metadata_line::parse_markdown_metadata_line;
use crate::table_of_contents::TableOfContents;
use crate::table_of_contents_heading::TableOfContentsHeading;

fn render_metadata_line_item(metadata_line_item: &MetadataLineItem) -> String {
    match metadata_line_item {
        MetadataLineItem::Flag { name } => format!(" {name}"),
        MetadataLineItem::Pair { name, value } => format!(
            " data-meta-{}=\"{}\"",
            escape_html(name),
            escape_html_attribute(value)
        ),
    }
}

fn render_code_metadata(metadata_line: &str) -> Result<String, ContentError> {
    parse_markdown_metadata_line(metadata_line).map(|metadata_line_items| {
        let rendered_items: String = metadata_line_items
            .iter()
            .map(render_metadata_line_item)
            .collect();

        format!(
            " data-meta-line=\"{}\"{rendered_items}",
            escape_html_attribute(metadata_line)
        )
    })
}

fn highlight_code(
    language: &str,
    syntax: &SyntaxReference,
    syntax_set: &SyntaxSet,
    code: &str,
) -> Result<String, ContentError> {
    let mut html_generator =
        ClassedHTMLGenerator::new_with_class_style(syntax, syntax_set, ClassStyle::Spaced);

    LinesWithEndings::from(code)
        .try_for_each(|line| html_generator.parse_html_for_line_which_includes_newline(line))
        .map_err(|source| ContentError::HighlightCode {
            language: language.to_owned(),
            source,
        })
        .map(|()| html_generator.finalize())
}

pub struct ContentDocumentEvaluator<'evaluator> {
    pub component_context: &'evaluator ContentDocumentComponentContext,
    pub rhai_template_renderer: &'evaluator RhaiTemplateRenderer,
    pub syntax_set: &'evaluator SyntaxSet,
}

impl ContentDocumentEvaluator<'_> {
    pub fn eval(&self, mdast: &Node) -> Result<String, ContentError> {
        match mdast {
            Node::Blockquote(Blockquote { children, .. }) => self.wrap("blockquote", children),
            Node::Break(_) => Ok("<br>".to_owned()),
            Node::Code(Code {
                lang, meta, value, ..
            }) => self.eval_code(lang.as_deref(), meta.as_deref(), value),
            Node::Delete(Delete { children, .. }) => self.wrap("del", children),
            Node::Emphasis(Emphasis { children, .. }) => self.wrap("em", children),
            Node::FootnoteReference(FootnoteReference {
                identifier, label, ..
            }) => Ok(format!(
                "<a href=\"#footnote-{identifier}\" role=\"doc-noteref\">{}</a>",
                label.as_ref().unwrap_or(identifier)
            )),
            Node::Heading(Heading {
                children, depth, ..
            }) => Ok(format!(
                "<h{depth} id=\"{}\">{}</h{depth}>",
                escape_html_attribute(&mdast_children_to_heading_id(children)),
                self.eval_children(children)?
            )),
            Node::Html(Html { value, .. }) | Node::Text(Text { value, .. }) => Ok(value.clone()),
            Node::Image(Image {
                alt, title, url, ..
            }) => self.eval_image(alt, title.as_deref(), url),
            Node::InlineCode(InlineCode { value, .. }) => {
                Ok(format!("<code>{}</code>", escape_html(value)))
            }
            Node::Link(Link {
                children,
                title,
                url,
                ..
            }) => self.eval_link(children, title.as_deref(), url),
            Node::List(List {
                children, ordered, ..
            }) => self.wrap(if *ordered { "ol" } else { "ul" }, children),
            Node::ListItem(ListItem { children, .. }) => self.wrap("li", children),
            Node::MdxFlowExpression(MdxFlowExpression { value, .. })
            | Node::MdxTextExpression(MdxTextExpression { value, .. }) => self
                .rhai_template_renderer
                .render_expression(self.component_context.clone(), value)
                .map(|evaluated_expression| evaluated_expression.to_string())
                .map_err(|source| ContentError::EvaluateExpression {
                    expression: value.clone(),
                    source,
                }),
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
            }) => eval_mdx_element(
                attributes,
                children,
                self.component_context,
                self.eval_children(children)?,
                name.as_ref(),
                self.rhai_template_renderer,
            )
            .map_err(ContentError::EvaluateMdxElement),
            Node::Paragraph(Paragraph { children, .. }) => self.wrap("p", children),
            Node::Root(Root { children, .. }) => self.eval_children(children),
            Node::Strong(Strong { children, .. }) => self.wrap("strong", children),
            Node::Table(Table { children, .. }) => self.wrap("table", children),
            Node::TableCell(TableCell { children, .. }) => self.wrap("td", children),
            Node::TableRow(TableRow { children, .. }) => self.wrap("tr", children),
            Node::ThematicBreak(_) => Ok("<hr>".to_owned()),
            Node::Toml(_) => Ok(String::new()),
            unsupported_node @ (Node::Definition(_)
            | Node::FootnoteDefinition(_)
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

    pub fn eval_children(&self, children: &[Node]) -> Result<String, ContentError> {
        children.iter().map(|child| self.eval(child)).collect()
    }

    pub fn table_of_contents(&self, mdast: &Node) -> Result<TableOfContents, ContentError> {
        let mut headings: Vec<TableOfContentsHeading> = vec![];

        self.collect_headings(mdast, &mut headings)?;

        Ok(TableOfContents { headings })
    }

    fn collect_headings(
        &self,
        mdast: &Node,
        headings: &mut Vec<TableOfContentsHeading>,
    ) -> Result<(), ContentError> {
        match mdast {
            Node::Heading(Heading {
                children, depth, ..
            }) => {
                headings.push(TableOfContentsHeading {
                    content: self.eval_children(children)?,
                    depth: i64::from(*depth),
                    id: mdast_children_to_heading_id(children),
                });

                Ok(())
            }
            Node::Paragraph(Paragraph { children, .. }) => {
                self.collect_headings_in_children(children, headings)
            }
            other_node => self.collect_headings_in_children(
                mdast_container_children(other_node).map_or(&[], Vec::as_slice),
                headings,
            ),
        }
    }

    fn collect_headings_in_children(
        &self,
        children: &[Node],
        headings: &mut Vec<TableOfContentsHeading>,
    ) -> Result<(), ContentError> {
        children
            .iter()
            .try_for_each(|child| self.collect_headings(child, headings))
    }

    fn eval_code(
        &self,
        language: Option<&str>,
        metadata_line: Option<&str>,
        code: &str,
    ) -> Result<String, ContentError> {
        let language_attributes = language.map_or_else(
            || "\"".to_owned(),
            |language| {
                format!(
                    " language-{0}\" data-lang=\"{0}\"",
                    escape_html_attribute(language)
                )
            },
        );
        let metadata_attributes = metadata_line
            .map(render_code_metadata)
            .transpose()?
            .unwrap_or_default();
        let rendered_code = language.map_or_else(
            || Ok(escape_html(code)),
            |language| {
                self.syntax_set.find_syntax_by_token(language).map_or_else(
                    || {
                        warn!("No syntax found for language: {language}");

                        Ok(escape_html(code))
                    },
                    |syntax| highlight_code(language, syntax, self.syntax_set, code),
                )
            },
        );

        rendered_code.map(|rendered_code| {
            format!(
                "<pre class=\"code{language_attributes}{metadata_attributes}><code>{rendered_code}</code></pre>"
            )
        })
    }

    fn eval_image(
        &self,
        alt: &str,
        title: Option<&str>,
        url: &str,
    ) -> Result<String, ContentError> {
        let image_source = self
            .component_context
            .asset_manager
            .image_source(url)
            .map_err(ContentError::ResolveImage)?;
        let title_attribute = title.map_or_else(String::new, |title| {
            format!(" title=\"{}\"", escape_html_attribute(title))
        });

        Ok(format!(
            "<img alt=\"{}\" src=\"{}\"{title_attribute}>",
            escape_html_attribute(alt),
            escape_html_attribute(&image_source)
        ))
    }

    fn eval_link(
        &self,
        children: &[Node],
        title: Option<&str>,
        url: &str,
    ) -> Result<String, ContentError> {
        let link = self
            .component_context
            .site
            .content_document_linker
            .resolve_link(url)?;
        let title_attribute = title.map_or_else(String::new, |title| {
            format!(" title=\"{}\"", escape_html_attribute(title))
        });

        Ok(format!(
            "<a href=\"{}\"{title_attribute}>{}</a>",
            escape_html_attribute(&link),
            self.eval_children(children)?
        ))
    }

    fn wrap(&self, tag: &str, children: &[Node]) -> Result<String, ContentError> {
        Ok(format!("<{tag}>{}</{tag}>", self.eval_children(children)?))
    }
}
