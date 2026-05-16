use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::EvalContext;
use rhai::Expression;

use super::expression_reference::ExpressionReference;

fn lookup_expression<'collection, 'expression>(
    expressions: &'collection [Expression<'expression>],
    expression_index: usize,
) -> Result<&'collection Expression<'expression>, Box<EvalAltResult>> {
    expressions.get(expression_index).ok_or_else(|| {
        Box::new(EvalAltResult::ErrorRuntime(
            "Expression index out of bounds".into(),
            rhai::Position::NONE,
        ))
    })
}

pub struct ExpressionCollection<'expression> {
    pub expressions: Vec<Expression<'expression>>,
}

impl<'expression> ExpressionCollection<'expression> {
    pub fn eval_expression(
        &mut self,
        eval_context: &mut EvalContext,
        ExpressionReference { expression_index }: &ExpressionReference,
    ) -> Result<Dynamic, Box<EvalAltResult>> {
        lookup_expression(&self.expressions, *expression_index)
            .and_then(|expression| eval_context.eval_expression_tree(expression))
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use anyhow::Result;
    use rhai::Dynamic;
    use rhai::EvalAltResult;
    use rhai::Expression;
    use rhai::Position;

    use super::lookup_expression;

    #[test]
    fn lookup_expression_returns_runtime_error_when_index_is_out_of_bounds() -> Result<()> {
        let expressions: Vec<Expression<'_>> = Vec::new();
        let reference = discriminant(&EvalAltResult::ErrorRuntime(Dynamic::UNIT, Position::NONE));

        assert!(lookup_expression(&expressions, 0).is_err_and(|boxed| {
            discriminant(boxed.as_ref()) == reference
                && boxed.to_string().contains("Expression index out of bounds")
        }));

        Ok(())
    }
}
