use rhai::EvalAltResult;

#[derive(Debug, thiserror::Error)]
pub enum RhaiComponentsError {
    #[error("calling the template function of component '{component_name}' failed")]
    CallTemplateFunction {
        component_name: String,
        #[source]
        source: Box<EvalAltResult>,
    },
    #[error("clsx expects boolean values, but class '{class_name}' maps to {value_type}")]
    ClsxValueNotBoolean {
        class_name: String,
        value_type: String,
    },
    #[error("component '{component_name}' exceeds the maximum nesting depth of {maximum_depth}")]
    ComponentNestingTooDeep {
        component_name: String,
        maximum_depth: usize,
    },
    #[error("'{variable_name}' variable not found in scope")]
    ComponentContextNotInScope { variable_name: String },
    #[error("expression failed: '{expression}'")]
    EvaluateExpression {
        expression: String,
        #[source]
        source: Box<EvalAltResult>,
    },
    #[error("expression index {expression_index} out of bounds")]
    ExpressionIndexOutOfBounds { expression_index: usize },
    #[error("component nesting depth is missing from the evaluation state")]
    MissingComponentNestingDepth,
    #[error("unable to resolve the module of component '{component_name}'")]
    ResolveComponentModule {
        component_name: String,
        #[source]
        source: Box<EvalAltResult>,
    },
    #[error("template '{name}' not found")]
    TemplateNotFound { name: String },
    #[error("expected a parsed tag stack in the component syntax state")]
    UnexpectedComponentState,
}

impl From<RhaiComponentsError> for Box<EvalAltResult> {
    fn from(rhai_components_error: RhaiComponentsError) -> Self {
        Self::new(EvalAltResult::ErrorSystem(
            String::new(),
            Box::new(rhai_components_error),
        ))
    }
}
