use anyhow::Result;
use anyhow::anyhow;
use log::warn;
use markdown::mdast::AttributeContent;
use markdown::mdast::AttributeValue;
use markdown::mdast::AttributeValueExpression;
use markdown::mdast::Blockquote;
use markdown::mdast::Code;
use markdown::mdast::Emphasis;
use markdown::mdast::Heading;
use markdown::mdast::Html;
use markdown::mdast::Link;
use markdown::mdast::List;
use markdown::mdast::ListItem;
use markdown::mdast::MdxJsxAttribute;
use markdown::mdast::MdxJsxFlowElement;
use markdown::mdast::Node;
use markdown::mdast::Paragraph;
use markdown::mdast::Root;
use markdown::mdast::Text;
use markdown::mdast::ThematicBreak;
use markdown::mdast::Toml;
use rhai::Dynamic;
use syntect::html::ClassStyle;
use syntect::html::ClassedHTMLGenerator;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use crate::escape_html::escape_html;
use crate::rhai_component_context::RhaiComponentContext;
use crate::rhai_template_renderer::RhaiTemplateRenderer;

pub fn eval_mdast(
    mdast: &Node,
    rhai_component_context: &RhaiComponentContext,
    rhai_template_renderer: &RhaiTemplateRenderer,
    syntax_set: &SyntaxSet,
) -> Result<String> {
    let mut result = String::new();

    match mdast {
        Node::Blockquote(Blockquote { children, .. }) => {
            result.push_str("<blockquote>");

            for child in children {
                result.push_str(&eval_mdast(
                    child,
                    rhai_component_context,
                    rhai_template_renderer,
                    syntax_set,
                )?);
            }

            result.push_str("</blockquote>");
        }
        Node::Code(Code {
            meta, lang, value, ..
        }) => {
            result.push_str(&format!(
                r#"<pre class="code{}"{}><code>"#,
                match lang {
                    Some(lang) => format!(" language-{}", lang),
                    None => "".to_string(),
                },
                match meta {
                    Some(meta) => format!(r#" data-meta="{}""#, escape_html(meta)),
                    None => "".to_string(),
                }
            ));
            if let Some(lang) = lang {
                let syntax = syntax_set.find_syntax_by_extension(lang);

                match syntax {
                    Some(syntax) => {
                        let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
                            syntax,
                            syntax_set,
                            ClassStyle::Spaced,
                        );
                        for line in LinesWithEndings::from(value) {
                            html_generator.parse_html_for_line_which_includes_newline(line)?;
                        }
                        let html_rs = html_generator.finalize();

                        result.push_str(&html_rs);
                    }
                    None => {
                        warn!("No syntax found for language: {}", lang);

                        result.push_str(&escape_html(value));
                    }
                }
            } else {
                result.push_str(&escape_html(value));
            }

            result.push_str("</code></pre>");
        }
        Node::Emphasis(Emphasis { children, .. }) => {
            result.push_str("<em>");

            for child in children {
                result.push_str(&eval_mdast(
                    child,
                    rhai_component_context,
                    rhai_template_renderer,
                    syntax_set,
                )?);
            }

            result.push_str("</em>");
        }
        Node::Heading(Heading {
            children, depth, ..
        }) => {
            let tag = format!("h{}", depth);
            result.push_str(&format!("<{}>", tag));

            for child in children {
                result.push_str(&eval_mdast(
                    child,
                    rhai_component_context,
                    rhai_template_renderer,
                    syntax_set,
                )?);
            }

            result.push_str(&format!("</{}>", tag));
        }
        Node::Html(Html { value, .. }) => {
            result.push_str(value);
        }
        Node::Link(Link {
            children,
            title,
            url,
            ..
        }) => {
            result.push_str(&format!("<a href=\"{}\"", url));

            if let Some(title) = title {
                result.push_str(&format!(" title=\"{}\"", title));
            }

            result.push('>');

            for child in children {
                result.push_str(&eval_mdast(
                    child,
                    rhai_component_context,
                    rhai_template_renderer,
                    syntax_set,
                )?);
            }

            result.push_str("</a>");
        }
        Node::List(List {
            children, ordered, ..
        }) => {
            if *ordered {
                result.push_str("<ol>");
            } else {
                result.push_str("<ul>");
            }

            for child in children {
                result.push_str(&eval_mdast(
                    child,
                    rhai_component_context,
                    rhai_template_renderer,
                    syntax_set,
                )?);
            }

            if *ordered {
                result.push_str("</ol>");
            } else {
                result.push_str("</ul>");
            }
        }
        Node::ListItem(ListItem { children, .. }) => {
            result.push_str("<li>");

            for child in children {
                result.push_str(&eval_mdast(
                    child,
                    rhai_component_context,
                    rhai_template_renderer,
                    syntax_set,
                )?);
            }

            result.push_str("</li>");
        }
        Node::MdxJsxFlowElement(MdxJsxFlowElement {
            attributes,
            children,
            name,
            ..
        }) => {
            result.push_str(
                &rhai_template_renderer.render(
                    &name
                        .clone()
                        .ok_or_else(|| anyhow!("MdxJsxFlowElement without a name"))?,
                    rhai_component_context.clone(),
                    Dynamic::from_map({
                        let mut props = rhai::Map::new();

                        for attribute in attributes {
                            match attribute {
                                AttributeContent::Expression(_) => {
                                    return Err(anyhow!(
                                        "Attribute expressions in Markdown are not supported"
                                    ));
                                }
                                AttributeContent::Property(MdxJsxAttribute { name, value }) => {
                                    props.insert(
                                        name.into(),
                                        match value {
                                            Some(value) => match value {
                                                AttributeValue::Literal(literal) => literal.into(),
                                                AttributeValue::Expression(
                                                    AttributeValueExpression { value, .. },
                                                ) => rhai_template_renderer.render_expression(
                                                    rhai_component_context.clone(),
                                                    value,
                                                )?,
                                            },
                                            None => true.into(),
                                        },
                                    );
                                }
                            }
                        }

                        props
                    }),
                    Dynamic::from({
                        let mut content = String::new();

                        for child in children {
                            content.push_str(&eval_mdast(
                                child,
                                rhai_component_context,
                                rhai_template_renderer,
                                syntax_set,
                            )?);
                        }

                        content
                    }),
                )?,
            );
        }
        Node::Paragraph(Paragraph { children, .. }) => {
            result.push_str("<p>");

            for child in children {
                result.push_str(&eval_mdast(
                    child,
                    rhai_component_context,
                    rhai_template_renderer,
                    syntax_set,
                )?);
            }

            result.push_str("</p>");
        }
        Node::Root(Root { children, .. }) => {
            for child in children {
                result.push_str(&eval_mdast(
                    child,
                    rhai_component_context,
                    rhai_template_renderer,
                    syntax_set,
                )?);
            }
        }
        Node::Text(Text { value, .. }) => {
            result.push_str(value);
        }
        Node::ThematicBreak(ThematicBreak { .. }) => {
            result.push_str("<hr>");
        }
        Node::Toml(Toml { .. }) => {
            // ignore frontmatter during this pass
        }
        item => {
            warn!("Unhandled node type: {:?}", item);
        }
    }

    Ok(result)
}
