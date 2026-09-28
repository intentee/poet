use rhai::EvalAltResult;
use rhai::Scope;
use rhai::Variant;

use crate::content_script_engine::content_script_engine;

pub fn evaluate_content_script<TScopeValue, TResult>(
    variable_name: &str,
    scope_value: TScopeValue,
    script: &str,
) -> Result<TResult, Box<EvalAltResult>>
where
    TScopeValue: Clone + Variant,
    TResult: Clone + Variant,
{
    let mut scope = Scope::new();

    scope.push(variable_name.to_owned(), scope_value);

    content_script_engine().eval_with_scope::<TResult>(&mut scope, script)
}
