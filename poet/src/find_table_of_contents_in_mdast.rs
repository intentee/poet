use anyhow::Result;
use markdown::mdast::Blockquote;
use markdown::mdast::Delete;
use markdown::mdast::Emphasis;
use markdown::mdast::Heading as MdastHeading;
use markdown::mdast::Link;
use markdown::mdast::List;
use markdown::mdast::ListItem;
use markdown::mdast::MdxJsxFlowElement;
use markdown::mdast::MdxJsxTextElement;
use markdown::mdast::Node;
use markdown::mdast::Paragraph;
use markdown::mdast::Root;
use markdown::mdast::Strong;
use markdown::mdast::Table;
use markdown::mdast::TableCell;
use markdown::mdast::TableRow;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;
use syntect::parsing::SyntaxSet;

use crate::content_document_component_context::ContentDocumentComponentContext;
use crate::eval_content_document_mdast::eval_content_document_children;
use crate::mdast_children_to_heading_id::mdast_children_to_heading_id;
use crate::table_of_contents::TableOfContents;
use crate::table_of_contents::heading::Heading;

fn find_headings_in_mdast(
    mdast: &Node,
    component_context: &ContentDocumentComponentContext,
    headings: &mut Vec<Heading>,
    rhai_template_renderer: &RhaiTemplateRenderer,
    syntax_set: &SyntaxSet,
) -> Result<()> {
    match mdast {
        Node::Blockquote(Blockquote { children, .. })
        | Node::Delete(Delete { children, .. })
        | Node::Emphasis(Emphasis { children, .. })
        | Node::Link(Link { children, .. })
        | Node::List(List { children, .. })
        | Node::ListItem(ListItem { children, .. })
        | Node::MdxJsxFlowElement(MdxJsxFlowElement { children, .. })
        | Node::MdxJsxTextElement(MdxJsxTextElement { children, .. })
        | Node::Paragraph(Paragraph { children, .. })
        | Node::Root(Root { children, .. })
        | Node::Strong(Strong { children, .. })
        | Node::Table(Table { children, .. })
        | Node::TableCell(TableCell { children, .. })
        | Node::TableRow(TableRow { children, .. }) => {
            for child in children {
                find_headings_in_mdast(
                    child,
                    component_context,
                    headings,
                    rhai_template_renderer,
                    syntax_set,
                )?;
            }

            Ok(())
        }
        Node::Heading(MdastHeading {
            children, depth, ..
        }) => {
            headings.push(Heading {
                content: eval_content_document_children(
                    children,
                    component_context,
                    rhai_template_renderer,
                    syntax_set,
                )?,
                depth: *depth as i64,
                id: mdast_children_to_heading_id(children)?,
            });

            Ok(())
        }
        _ => Ok(()),
    }
}

pub fn find_table_of_contents_in_mdast(
    mdast: &Node,
    component_context: &ContentDocumentComponentContext,
    rhai_template_renderer: &RhaiTemplateRenderer,
    syntax_set: &SyntaxSet,
) -> Result<TableOfContents> {
    let mut headings: Vec<Heading> = Vec::new();

    find_headings_in_mdast(
        mdast,
        component_context,
        &mut headings,
        rhai_template_renderer,
        syntax_set,
    )?;

    Ok(TableOfContents { headings })
}
