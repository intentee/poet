use anyhow::Result;
use markdown::mdast::Heading as MdastHeading;
use markdown::mdast::Node;
use markdown::mdast::Paragraph;
use poet_mdx::mdast_container_children::mdast_container_children;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;
use syntect::parsing::SyntaxSet;

use crate::content_document_component_context::ContentDocumentComponentContext;
use crate::eval_content_document_mdast::eval_content_document_children;
use crate::mdast_children_to_heading_id::mdast_children_to_heading_id;
use crate::table_of_contents::TableOfContents;
use crate::table_of_contents::heading::Heading;

fn find_headings_in_children(
    children: &[Node],
    component_context: &ContentDocumentComponentContext,
    headings: &mut Vec<Heading>,
    rhai_template_renderer: &RhaiTemplateRenderer,
    syntax_set: &SyntaxSet,
) -> Result<()> {
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

fn find_headings_in_mdast(
    mdast: &Node,
    component_context: &ContentDocumentComponentContext,
    headings: &mut Vec<Heading>,
    rhai_template_renderer: &RhaiTemplateRenderer,
    syntax_set: &SyntaxSet,
) -> Result<()> {
    match mdast {
        Node::Paragraph(Paragraph { children, .. }) => find_headings_in_children(
            children,
            component_context,
            headings,
            rhai_template_renderer,
            syntax_set,
        ),
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
                id: mdast_children_to_heading_id(children),
            });

            Ok(())
        }
        other_node => find_headings_in_children(
            mdast_container_children(other_node).map_or(&[], Vec::as_slice),
            component_context,
            headings,
            rhai_template_renderer,
            syntax_set,
        ),
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use poet_mdx::string_to_mdast::string_to_mdast;
    use rhai::Engine;
    use rhai_components::component_syntax::component_registry::ComponentRegistry;
    use rhai_components::rhai_template_renderer_params::RhaiTemplateRendererParams;
    use syntect::parsing::SyntaxSet;

    use super::*;

    #[test]
    fn extracts_headings_with_depth_and_id() -> Result<()> {
        let component_context = ContentDocumentComponentContext::mock();
        let rhai_template_renderer = RhaiTemplateRenderer::build(RhaiTemplateRendererParams {
            component_registry: Arc::new(ComponentRegistry::default()),
            expression_engine: Engine::new_raw(),
        })?;
        let syntax_set = SyntaxSet::new();
        let mdast = string_to_mdast("# First Heading\n\nbody text\n\n## Second Heading")?;

        let table_of_contents = find_table_of_contents_in_mdast(
            &mdast,
            &component_context,
            &rhai_template_renderer,
            &syntax_set,
        )?;

        assert_eq!(table_of_contents.headings.len(), 2);
        assert!(
            table_of_contents.headings[0]
                .content
                .contains("First Heading")
        );
        assert_eq!(table_of_contents.headings[0].depth, 1);
        assert_eq!(table_of_contents.headings[0].id, "first-heading");
        assert_eq!(table_of_contents.headings[1].depth, 2);
        assert_eq!(table_of_contents.headings[1].id, "second-heading");

        Ok(())
    }
}
