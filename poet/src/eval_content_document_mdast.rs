use anyhow::Result;
use anyhow::anyhow;
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
use markdown::mdast::ThematicBreak;
use rhai_components::escape_html::escape_html;
use rhai_components::escape_html_attribute::escape_html_attribute;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;
use syntect::html::ClassStyle;
use syntect::html::ClassedHTMLGenerator;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use crate::content_document_component_context::ContentDocumentComponentContext;
use crate::eval_mdx_element::eval_mdx_element;
use crate::is_external_link::is_external_link;
use crate::mdast_children_to_heading_id::mdast_children_to_heading_id;
use crate::parse_markdown_metadata_line::metadata_line_item::MetadataLineItem;
use crate::parse_markdown_metadata_line::parse_markdown_metadata_line;

pub fn eval_content_document_children(
    children: &Vec<Node>,
    component_context: &ContentDocumentComponentContext,
    rhai_template_renderer: &RhaiTemplateRenderer,
    syntax_set: &SyntaxSet,
) -> Result<String> {
    let mut content = String::new();

    for child in children {
        content.push_str(&eval_content_document_mdast(
            child,
            component_context,
            rhai_template_renderer,
            syntax_set,
        )?);
    }

    Ok(content)
}

pub fn eval_content_document_mdast(
    mdast: &Node,
    component_context: &ContentDocumentComponentContext,
    rhai_template_renderer: &RhaiTemplateRenderer,
    syntax_set: &SyntaxSet,
) -> Result<String> {
    let mut result = String::new();

    match mdast {
        Node::Blockquote(Blockquote { children, .. }) => {
            result.push_str("<blockquote>");
            result.push_str(&eval_content_document_children(
                children,
                component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
            result.push_str("</blockquote>");
        }
        Node::Break(_) => {
            result.push_str("<br>");
        }
        Node::Code(Code {
            meta, lang, value, ..
        }) => {
            result.push_str("<pre class=\"code");

            if let Some(lang) = lang {
                result.push_str(&format!(" language-{lang}\""));
                result.push_str(&format!(" data-lang=\"{}\"", escape_html_attribute(lang)));
            } else {
                result.push('"');
            }

            if let Some(meta) = meta {
                result.push_str(&format!(
                    r#" data-meta-line="{}""#,
                    escape_html_attribute(meta)
                ));

                for item in parse_markdown_metadata_line(meta)? {
                    match item {
                        MetadataLineItem::Flag { name } => {
                            result.push_str(&format!(" {name}"));
                        }
                        MetadataLineItem::Pair { name, value } => {
                            result.push_str(&format!(
                                " data-meta-{}=\"{}\"",
                                escape_html(&name),
                                escape_html_attribute(&value)
                            ));
                        }
                    }
                }
            }

            result.push_str("><code>");

            if let Some(lang) = lang {
                let syntax = syntax_set.find_syntax_by_token(lang);

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
        Node::Definition(node) => {
            warn!("Definitions are not supported: {node:?}");
        }
        Node::Delete(Delete { children, .. }) => {
            result.push_str("<del>");
            result.push_str(&eval_content_document_children(
                children,
                component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
            result.push_str("</del>");
        }
        Node::Emphasis(Emphasis { children, .. }) => {
            result.push_str("<em>");
            result.push_str(&eval_content_document_children(
                children,
                component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
            result.push_str("</em>");
        }
        Node::FootnoteDefinition(node) => {
            warn!("Footnote definitions are not supported: {node:?}");
        }
        Node::FootnoteReference(FootnoteReference {
            identifier, label, ..
        }) => {
            result.push_str(&format!(
                "<a href=\"#footnote-{}\" role=\"doc-noteref\">{}</a>",
                identifier,
                if let Some(label) = label {
                    label
                } else {
                    identifier
                },
            ));
        }
        Node::Heading(Heading {
            children, depth, ..
        }) => {
            let tag = format!("h{}", depth);

            result.push_str(&format!(
                "<{} id=\"{}\">",
                tag,
                escape_html_attribute(&mdast_children_to_heading_id(children)?)
            ));
            result.push_str(&eval_content_document_children(
                children,
                component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
            result.push_str(&format!("</{}>", tag));
        }
        Node::Html(Html { value, .. }) => {
            result.push_str(value);
        }
        Node::Image(Image {
            alt, url, title, ..
        }) => {
            result.push_str(&format!("<img alt=\"{}\" ", escape_html_attribute(alt)));

            let src = if is_external_link(url) {
                url
            } else {
                &match component_context.asset_manager.file(url) {
                    Ok(src) => src,
                    Err(err) => return Err(anyhow!(err)),
                }
            };

            result.push_str(&format!("src=\"{}\"", escape_html_attribute(src)));

            if let Some(title) = title {
                result.push_str(&format!(" title=\"{}\"", escape_html_attribute(title)));
            }

            result.push('>');
        }
        Node::ImageReference(node) => {
            warn!("Image references are not supported: {node:?}");
        }
        Node::InlineCode(InlineCode { value, .. }) => {
            result.push_str(&format!("<code>{}</code>", escape_html_attribute(value)));
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
            let link = if is_external_link(url) {
                url.clone()
            } else {
                match component_context.content_document_linker.link_to(url) {
                    Ok(link) => link,
                    Err(err) => return Err(anyhow!(err)),
                }
            };

            result.push_str(&format!("<a href=\"{link}\""));

            if let Some(title) = title {
                result.push_str(&format!(" title=\"{}\"", title));
            }

            result.push('>');
            result.push_str(&eval_content_document_children(
                children,
                component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
            result.push_str("</a>");
        }
        Node::LinkReference(node) => {
            warn!("Link references are not supported: {node:?}");
        }
        Node::List(List {
            children, ordered, ..
        }) => {
            if *ordered {
                result.push_str("<ol>");
            } else {
                result.push_str("<ul>");
            }

            result.push_str(&eval_content_document_children(
                children,
                component_context,
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
            result.push_str(&eval_content_document_children(
                children,
                component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
            result.push_str("</li>");
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
                eval_content_document_children(
                    children,
                    component_context,
                    rhai_template_renderer,
                    syntax_set,
                )?,
                name,
                rhai_template_renderer,
            )?);
        }
        Node::Paragraph(Paragraph { children, .. }) => {
            result.push_str("<p>");
            result.push_str(&eval_content_document_children(
                children,
                component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
            result.push_str("</p>");
        }
        Node::Root(Root { children, .. }) => {
            result.push_str(&eval_content_document_children(
                children,
                component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
        }
        Node::Strong(Strong { children, .. }) => {
            result.push_str("<strong>");
            result.push_str(&eval_content_document_children(
                children,
                component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
            result.push_str("</strong>");
        }
        Node::Table(Table { children, .. }) => {
            result.push_str("<table>");
            result.push_str(&eval_content_document_children(
                children,
                component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
            result.push_str("</table>");
        }
        Node::TableCell(TableCell { children, .. }) => {
            result.push_str("<td>");
            result.push_str(&eval_content_document_children(
                children,
                component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
            result.push_str("</td>");
        }
        Node::TableRow(TableRow { children, .. }) => {
            result.push_str("<tr>");
            result.push_str(&eval_content_document_children(
                children,
                component_context,
                rhai_template_renderer,
                syntax_set,
            )?);
            result.push_str("</tr>");
        }
        Node::Text(Text { value, .. }) => {
            result.push_str(value);
        }
        Node::ThematicBreak(ThematicBreak { .. }) => {
            result.push_str("<hr>");
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
mod tests {
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::str::FromStr;
    use std::sync::Arc;

    use esbuild_metafile::EsbuildMetaFile;
    use indoc::indoc;
    use rhai::Engine;
    use rhai_components::component_syntax::component_registry::ComponentRegistry;
    use rhai_components::rhai_template_renderer_params::RhaiTemplateRendererParams;
    use syntect::parsing::SyntaxDefinition;
    use syntect::parsing::SyntaxSetBuilder;

    use super::*;
    use crate::asset_manager::AssetManager;
    use crate::asset_path_renderer::AssetPathRenderer;
    use crate::author_collection::AuthorCollection;
    use crate::content_document_front_matter::ContentDocumentFrontMatter;
    use crate::content_document_linker::ContentDocumentLinker;
    use crate::content_document_reference::ContentDocumentReference;
    use crate::string_to_mdast::string_to_mdast;

    const ASSET_METAFILE: &str = indoc! {r#"
        {
            "outputs": {
                "static/logo_ABCDEF12.png": {
                    "imports": [],
                    "inputs": { "logo.png": {} }
                },
                "static/entry_ABCDEF12.js": {
                    "imports": [{ "path": "static/logo_ABCDEF12.png" }],
                    "entryPoint": "logo.png",
                    "inputs": {}
                }
            }
        }
    "#};

    fn asset_manager() -> Result<AssetManager> {
        Ok(AssetManager::from_esbuild_metafile(
            Arc::new(EsbuildMetaFile::from_str(ASSET_METAFILE)?),
            AssetPathRenderer {
                base_path: "/".to_string(),
            },
        ))
    }

    fn linker() -> ContentDocumentLinker {
        let mut content_document_by_basename = HashMap::new();

        content_document_by_basename.insert(
            "guide".to_string().into(),
            ContentDocumentReference {
                basename_path: "guide".into(),
                front_matter: ContentDocumentFrontMatter::mock("guide"),
                generated_page_base_path: "/".to_string(),
            },
        );

        ContentDocumentLinker {
            content_document_basename_by_id: Arc::new(HashMap::new()),
            content_document_by_basename: Arc::new(content_document_by_basename),
        }
    }

    fn context() -> Result<ContentDocumentComponentContext> {
        Ok(ContentDocumentComponentContext {
            asset_manager: asset_manager()?,
            authors: Vec::new(),
            available_authors: Arc::new(AuthorCollection::default()),
            available_collections: Arc::new(HashSet::new()),
            content_document_collections_ranked: Arc::new(HashMap::new()),
            content_document_linker: linker(),
            front_matter: ContentDocumentFrontMatter::mock("doc"),
            is_watching: false,
            reference: ContentDocumentReference {
                basename_path: "doc".into(),
                front_matter: ContentDocumentFrontMatter::mock("doc"),
                generated_page_base_path: "/".to_string(),
            },
            table_of_contents: None,
        })
    }

    fn renderer() -> Result<RhaiTemplateRenderer> {
        RhaiTemplateRenderer::build(RhaiTemplateRendererParams {
            component_registry: Arc::new(ComponentRegistry::default()),
            expression_engine: Engine::new_raw(),
        })
    }

    fn single_token_syntax_set() -> Result<SyntaxSet> {
        let definition = SyntaxDefinition::load_from_str(
            indoc! {r#"
                %YAML 1.2
                ---
                name: Demo
                file_extensions: [demo]
                scope: source.demo
                contexts:
                  main:
                    - match: '\bfn\b'
                      scope: keyword.demo
            "#},
            true,
            None,
        )?;
        let mut builder = SyntaxSetBuilder::new();

        builder.add(definition);

        Ok(builder.build())
    }

    fn render_with_syntax_set(markdown: &str, syntax_set: &SyntaxSet) -> Result<String> {
        eval_content_document_mdast(
            &string_to_mdast(markdown)?,
            &context()?,
            &renderer()?,
            syntax_set,
        )
    }

    fn render(markdown: &str) -> Result<String> {
        render_with_syntax_set(markdown, &SyntaxSet::new())
    }

    #[test]
    fn renders_heading_with_generated_id() -> Result<()> {
        assert_eq!(
            render("## Hello World")?,
            "<h2 id=\"hello-world\">Hello World</h2>"
        );

        Ok(())
    }

    #[test]
    fn renders_paragraph_with_emphasis_and_strong() -> Result<()> {
        assert_eq!(
            render("*one* **two**")?,
            "<p><em>one</em> <strong>two</strong></p>"
        );

        Ok(())
    }

    #[test]
    fn renders_inline_code() -> Result<()> {
        assert_eq!(render("`let x = 1`")?, "<p><code>let x = 1</code></p>");

        Ok(())
    }

    #[test]
    fn renders_strikethrough() -> Result<()> {
        assert_eq!(render("~~gone~~")?, "<p><del>gone</del></p>");

        Ok(())
    }

    #[test]
    fn renders_blockquote() -> Result<()> {
        assert_eq!(
            render("> quoted")?,
            "<blockquote><p>quoted</p></blockquote>"
        );

        Ok(())
    }

    #[test]
    fn renders_unordered_list() -> Result<()> {
        let rendered = render("- first\n- second")?;

        assert!(rendered.starts_with("<ul>"));
        assert!(rendered.ends_with("</ul>"));
        assert!(rendered.contains("first"));
        assert!(rendered.contains("second"));

        Ok(())
    }

    #[test]
    fn renders_ordered_list() -> Result<()> {
        let rendered = render("1. first\n2. second")?;

        assert!(rendered.starts_with("<ol>"));
        assert!(rendered.ends_with("</ol>"));

        Ok(())
    }

    #[test]
    fn renders_thematic_break() -> Result<()> {
        assert_eq!(render("***")?, "<hr>");

        Ok(())
    }

    #[test]
    fn renders_table() -> Result<()> {
        let rendered = render("| H1 | H2 |\n| -- | -- |\n| a | b |")?;

        assert!(rendered.starts_with("<table>"));
        assert!(rendered.ends_with("</table>"));
        assert!(rendered.contains("<tr><td>H1</td><td>H2</td></tr>"));
        assert!(rendered.contains("<tr><td>a</td><td>b</td></tr>"));

        Ok(())
    }

    #[test]
    fn highlights_code_block_for_known_language() -> Result<()> {
        let rendered =
            render_with_syntax_set("```demo\nfn main\n```", &single_token_syntax_set()?)?;

        assert!(
            rendered.starts_with("<pre class=\"code language-demo\" data-lang=\"demo\"><code>")
        );
        assert!(rendered.contains("<span"));
        assert!(rendered.ends_with("</code></pre>"));

        Ok(())
    }

    #[test]
    fn escapes_code_block_for_unknown_language() -> Result<()> {
        let rendered = render("```nosuchlang\na < b\n```")?;

        assert!(rendered.contains("language-nosuchlang"));
        assert!(!rendered.contains("<span"));
        assert!(rendered.contains("a &lt; b"));

        Ok(())
    }

    #[test]
    fn escapes_code_block_without_language() -> Result<()> {
        let rendered = render("```\na < b\n```")?;

        assert!(rendered.starts_with("<pre class=\"code\"><code>"));
        assert!(rendered.contains("a &lt; b"));
        assert!(rendered.ends_with("</code></pre>"));

        Ok(())
    }

    #[test]
    fn renders_external_link_with_title() -> Result<()> {
        assert_eq!(
            render("[label](https://example.com \"Tooltip\")")?,
            "<p><a href=\"https://example.com\" title=\"Tooltip\">label</a></p>"
        );

        Ok(())
    }

    #[test]
    fn resolves_internal_link_through_linker() -> Result<()> {
        assert_eq!(
            render("[label](guide)")?,
            "<p><a href=\"/guide/\">label</a></p>"
        );

        Ok(())
    }

    #[test]
    fn fails_internal_link_to_missing_document() {
        assert!(render("[label](ghost)").is_err());
    }

    #[test]
    fn renders_external_image() -> Result<()> {
        assert_eq!(
            render("![photo](https://example.com/p.png)")?,
            "<p><img alt=\"photo\" src=\"https://example.com/p.png\"></p>"
        );

        Ok(())
    }

    #[test]
    fn resolves_internal_image_through_asset_manager() -> Result<()> {
        assert_eq!(
            render("![logo](logo.png)")?,
            "<p><img alt=\"logo\" src=\"/static/logo_ABCDEF12.png\"></p>"
        );

        Ok(())
    }

    #[test]
    fn fails_internal_image_for_missing_asset() {
        assert!(render("![logo](missing.png)").is_err());
    }

    #[test]
    fn skips_unsupported_math_node() -> Result<()> {
        assert_eq!(render("$$\nx = 1\n$$")?, "");

        Ok(())
    }

    #[test]
    fn renders_hard_break() -> Result<()> {
        assert!(render("first\\\nsecond")?.contains("<br>"));

        Ok(())
    }

    #[test]
    fn renders_code_block_metadata_flags_and_pairs() -> Result<()> {
        let rendered = render("```text highlighted label:foo\ncode\n```")?;

        assert!(rendered.contains(r#"data-meta-line="highlighted label:foo""#));
        assert!(rendered.contains(" highlighted"));
        assert!(rendered.contains(r#"data-meta-label="foo""#));

        Ok(())
    }

    #[test]
    fn fails_on_invalid_code_block_metadata() {
        assert!(render("```text bad:\"unterminated\ncode\n```").is_err());
    }

    #[test]
    fn renders_external_image_with_title() -> Result<()> {
        assert_eq!(
            render("![photo](https://example.com/p.png \"Caption\")")?,
            "<p><img alt=\"photo\" src=\"https://example.com/p.png\" title=\"Caption\"></p>"
        );

        Ok(())
    }

    #[test]
    fn renders_footnote_reference() -> Result<()> {
        let rendered = render("note[^a]\n\n[^a]: detail")?;

        assert!(rendered.contains(r##"href="#footnote-a""##));
        assert!(rendered.contains(r#"role="doc-noteref""#));

        Ok(())
    }
}
