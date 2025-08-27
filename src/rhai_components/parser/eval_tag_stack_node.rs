use rhai::EvalAltResult;
use rhai::EvalContext;

use super::eval_tag::eval_tag;
use super::expression_collection::ExpressionCollection;
use super::tag_stack_node::TagStackNode;

pub fn eval_tag_stack_node<'node>(
    context: &mut EvalContext,
    current_node: &'node TagStackNode,
    expression_collection: &mut ExpressionCollection,
) -> Result<String, Box<EvalAltResult>> {
    match current_node {
        TagStackNode::BodyExpression(expression_reference) => {
            Ok(expression_collection.render_expression(context, expression_reference)?)
        }
        TagStackNode::Tag {
            children,
            is_closed,
            opening_tag,
        } => {
            let mut result = String::new();

            if let Some(opening_tag) = &opening_tag
                && !opening_tag.is_component()
            {
                result.push_str(&eval_tag(context, expression_collection, opening_tag)?);
            }

            for child in children {
                result.push_str(&eval_tag_stack_node(context, child, expression_collection)?);
            }

            if let Some(opening_tag) = &opening_tag
                && *is_closed
                && !opening_tag.is_component()
            {
                result.push_str(&format!("</{}>", opening_tag.name));
            }

            if let Some(opening_tag) = &opening_tag
                && opening_tag.is_component()
            {
                // context.global_runtime_state_mut().iter_imports().for_each(|(name, module)| {
                //     println!("imported module: {} {:#?}", name, module);
                // });
                context.iter_namespaces().for_each(|module| {
                    println!("regsitered namespace: {:#?}", module);
                });

                for (name, is_const, dynamic) in context.scope().iter() {
                    println!("scoped variable: {} {:#?} = {:#?}", name, is_const, dynamic);
                }

                // println!("Eval result: {:#?}", context.engine().eval::<Dynamic>("Note::template(1, 2, 3)")?);

                // context.call_fn(
                //     "template",
                //     (Dynamic::from(""), Dynamic::from(""), Dynamic::from("")),
                // )?;
            }

            Ok(result)
        }
        TagStackNode::Text(text) => Ok(text.clone()),
    }
}
