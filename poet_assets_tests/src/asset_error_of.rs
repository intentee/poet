use poet_assets::asset_error::AssetError;
use rhai::EvalAltResult;

#[must_use]
pub fn asset_error_of(eval_alt_result: &EvalAltResult) -> Option<&AssetError> {
    match eval_alt_result {
        EvalAltResult::ErrorSystem(_, source) => source.downcast_ref::<AssetError>(),
        _ => None,
    }
}
