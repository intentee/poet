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
use markdown::mdast::Image;
use markdown::mdast::InlineCode;
use markdown::mdast::Link;
use markdown::mdast::List;
use markdown::mdast::ListItem;
use markdown::mdast::MdxJsxAttribute;
use markdown::mdast::MdxJsxFlowElement;
use markdown::mdast::Node;
use markdown::mdast::Paragraph;
use markdown::mdast::Root;
use markdown::mdast::Strong;
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
use crate::rhai_components::tag_name::TagName;
use crate::rhai_template_renderer::RhaiTemplateRenderer;
use crate::string_to_mdast::string_to_mdast;

fn eval_children(
    children: &Vec<Node>,
    rhai_component_context: &RhaiComponentContext,
    rhai_template_renderer: &RhaiTemplateRenderer,
    syntax_set: &SyntaxSet,
) -> Result<String> {
    let mut content = String::new();

    for child in children {
        content.push_str(&eval_mdast(
            child,
            rhai_component_context,
            rhai_template_renderer,
            syntax_set,
        )?);
    }

    Ok(content)
}

/// JSX elements are initially parsed as Code blocks. They need to be converted to `mdast` again,
/// and re-evaluated.
fn eval_jsx_flow_element_children(
    children: &Vec<Node>,
    rhai_component_context: &RhaiComponentContext,
    rhai_template_renderer: &RhaiTemplateRenderer,
    syntax_set: &SyntaxSet,
) -> Result<String> {
    let mut result = String::new();

    for node in children {
        match node {
            Node::Code(Code { value, .. }) => {
                result.push_str(&eval_mdast(
                    &string_to_mdast(value)?,
                    rhai_component_context,
                    rhai_template_renderer,
                    syntax_set,
                )?);
            }
            _ => {
                return Err(anyhow!(
                    "Unexpected JSX child node. Expected only code nodes"
                ));
            }
        }
    }

    Ok(result)
}

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
            result.push_str(&eval_children(
                children,
                rhai_component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
            result.push_str("</blockquote>");
        }
        Node::Code(Code {
            meta, lang, value, ..
        }) => {
            result.push_str(&format!(
                r#"<pre class="code{}"{}{}><code>"#,
                match lang {
                    Some(lang) => format!(" language-{}", lang),
                    None => "".to_string(),
                },
                match lang {
                    Some(lang) => format!(" data-lang=\"{}\"", escape_html(lang)),
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
            result.push_str(&eval_children(
                children,
                rhai_component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
            result.push_str("</em>");
        }
        Node::Heading(Heading {
            children, depth, ..
        }) => {
            let tag = format!("h{}", depth);

            result.push_str(&format!("<{}>", tag));
            result.push_str(&eval_children(
                children,
                rhai_component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
            result.push_str(&format!("</{}>", tag));
        }
        Node::Html(Html { value, .. }) => {
            println!("HTML: {value}");
            result.push_str(value);
        }
        Node::Image(Image {
            alt, url, title, ..
        }) => {
            result.push_str(&format!("<img alt=\"{}\" ", escape_html(alt)));

            let src = if url.starts_with("http:") || url.starts_with("https:") {
                url
            } else {
                &match rhai_component_context.asset_manager.file(url) {
                    Ok(src) => src,
                    Err(err) => return Err(anyhow!(err)),
                }
            };

            result.push_str(&format!("src=\"{}\"", escape_html(src)));

            if let Some(title) = title {
                result.push_str(&format!(" title=\"{}\"", escape_html(title)));
            }

            result.push('>');
        }
        Node::InlineCode(InlineCode { value, .. }) => {
            result.push_str(&format!("<code>{}</code>", escape_html(value)));
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
            result.push_str(&eval_children(
                children,
                rhai_component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
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

            result.push_str(&eval_children(
                children,
                rhai_component_context,
                rhai_template_renderer,
                syntax_set,
            )?);

            if *ordered {
                result.push_str("</ol>");
            } else {
                result.push_str("</ul>");
            }
        }
        Node::ListItem(ListItem { children, .. }) => {
            result.push_str("<li>");
            result.push_str(&eval_children(
                children,
                rhai_component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
            result.push_str("</li>");
        }
        Node::MdxJsxFlowElement(MdxJsxFlowElement {
            attributes,
            children,
            name,
            ..
        }) => {
            let tag_name = TagName {
                name: name
                    .clone()
                    .ok_or_else(|| anyhow!("MdxJsxFlowElement without a name"))?,
            };

            let props = {
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
                                        AttributeValue::Expression(AttributeValueExpression {
                                            value,
                                            ..
                                        }) => rhai_template_renderer.render_expression(
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
            };

            if tag_name.is_void_element() && !children.is_empty() {
                return Err(anyhow!("Void element cannot have children"));
            }

            let evaluated_children = eval_jsx_flow_element_children(
                children,
                rhai_component_context,
                rhai_template_renderer,
                syntax_set,
            )?;

            if tag_name.is_component() {
                result.push_str(&rhai_template_renderer.render(
                    &tag_name.name,
                    rhai_component_context.clone(),
                    Dynamic::from_map(props),
                    Dynamic::from(evaluated_children),
                )?);
            } else {
                result.push_str(&format!("<{} ", tag_name.name));

                for (name, value) in props {
                    if value.is_bool() {
                        result.push_str(&format!("{name} "));
                    } else {
                        result
                            .push_str(&format!("{name}=\"{}\" ", escape_html(&value.to_string())));
                    }
                }

                result.push('>');

                if !children.is_empty() {
                    result.push_str(&evaluated_children);
                }

                if !children.is_empty() || !tag_name.is_void_element() {
                    result.push_str(&format!("</{}>", tag_name.name));
                }
            }
        }
        Node::Paragraph(Paragraph { children, .. }) => {
            result.push_str("<p>");
            result.push_str(&eval_children(
                children,
                rhai_component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
            result.push_str("</p>");
        }
        Node::Root(Root { children, .. }) => {
            result.push_str(&eval_children(
                children,
                rhai_component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
        }
        Node::Strong(Strong { children, .. }) => {
            result.push_str("<strong>");
            result.push_str(&eval_children(
                children,
                rhai_component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
            result.push_str("</strong>");
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
