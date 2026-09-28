use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::EvalContext;
use rhai::Expression;

use super::expression_reference::ExpressionReference;
use crate::rhai_components_error::RhaiComponentsError;

pub struct ExpressionCollection<'collection, 'expression> {
    pub expressions: &'collection [Expression<'expression>],
}

impl<'collection, 'expression> ExpressionCollection<'collection, 'expression> {
    pub fn expression(
        &self,
        ExpressionReference { expression_index }: &ExpressionReference,
    ) -> Result<&'collection Expression<'expression>, RhaiComponentsError> {
        self.expressions.get(*expression_index).ok_or(
            RhaiComponentsError::ExpressionIndexOutOfBounds {
                expression_index: *expression_index,
            },
        )
    }

    pub fn eval_expression(
        &self,
        eval_context: &mut EvalContext,
        expression_reference: &ExpressionReference,
    ) -> Result<Dynamic, Box<EvalAltResult>> {
        eval_context.eval_expression_tree(self.expression(expression_reference)?)
    }
}

#[cfg(test)]
mod tests {
    use super::ExpressionCollection;
    use crate::component_syntax::expression_reference::ExpressionReference;
    use crate::rhai_components_error::RhaiComponentsError;

    #[test]
    fn rejects_reference_beyond_collected_expressions() {
        let expression_collection = ExpressionCollection { expressions: &[] };

        assert!(
            expression_collection
                .expression(&ExpressionReference {
                    expression_index: 3
                })
                .is_err_and(|rhai_components_error| matches!(
                    rhai_components_error,
                    RhaiComponentsError::ExpressionIndexOutOfBounds {
                        expression_index: 3
                    }
                ))
        );
    }
}
