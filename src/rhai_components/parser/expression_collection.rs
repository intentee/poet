use rhai::EvalAltResult;
use rhai::EvalContext;
use rhai::Expression;

use super::expression_reference::ExpressionReference;

pub struct ExpressionCollection<'expression> {
    pub expressions: Vec<Expression<'expression>>,
}

impl<'expression> ExpressionCollection<'expression> {
    pub fn render_expression(
        &mut self,
        context: &mut EvalContext,
        ExpressionReference { expression_index }: &ExpressionReference,
    ) -> Result<String, Box<EvalAltResult>> {
        let expression = self.expressions.get(*expression_index).ok_or_else(|| {
            Box::new(EvalAltResult::ErrorRuntime(
                "Expression index out of bounds".into(),
                rhai::Position::NONE,
            ))
        })?;

        Ok(context.eval_expression_tree(expression)?.into_string()?)
    }
}
