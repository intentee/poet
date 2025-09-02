use anyhow::Result;
use markdown::mdast::Heading as MdastHeading;
use markdown::mdast::Node;
use markdown::mdast::Root;
use syntect::parsing::SyntaxSet;

use crate::component_context::ComponentContext;
use crate::eval_mdast::eval_children;
use crate::mdast_children_to_heading_id::mdast_children_to_heading_id;
use crate::rhai_template_renderer::RhaiTemplateRenderer;
use crate::table_of_contents::TableOfContents;
use crate::table_of_contents::heading::Heading;

pub fn find_headings_in_mdast(
    mdast: &Node,
    component_context: &ComponentContext,
    headings: &mut Vec<Heading>,
    rhai_template_renderer: &RhaiTemplateRenderer,
    syntax_set: &SyntaxSet,
) -> Result<()> {
    match mdast {
        Node::Root(Root { children, .. }) => {
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
                content: eval_children(
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
    component_context: &ComponentContext,
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
