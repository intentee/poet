use rhai::EvalAltResult;
use rhai::Map;

use crate::rhai_components_error::RhaiComponentsError;

pub fn clsx(class_toggles: Map) -> Result<String, Box<EvalAltResult>> {
    let mut glued_class = String::new();

    for (class_name, class_toggle) in &class_toggles {
        let is_enabled = class_toggle.as_bool().map_err(|value_type| {
            RhaiComponentsError::ClsxValueNotBoolean {
                class_name: class_name.to_string(),
                value_type: value_type.to_owned(),
            }
        })?;

        if is_enabled {
            glued_class.push(' ');
            glued_class.push_str(class_name);
        }
    }

    Ok(glued_class.trim().to_owned())
}

#[cfg(test)]
mod tests {
    use rhai::Dynamic;
    use rhai::EvalAltResult;
    use rhai::Map;

    use super::clsx;
    use crate::rhai_components_error::RhaiComponentsError;

    fn class_toggles(entries: &[(&str, Dynamic)]) -> Map {
        entries
            .iter()
            .map(|(class_name, class_toggle)| ((*class_name).into(), class_toggle.clone()))
            .collect()
    }

    #[test]
    fn joins_enabled_classes() {
        assert!(
            clsx(class_toggles(&[
                ("a", Dynamic::from(true)),
                ("b", Dynamic::from(false)),
                ("c", Dynamic::from(true)),
            ]))
            .is_ok_and(|glued_class| glued_class == "a c")
        );
    }

    #[test]
    fn rejects_non_boolean_toggle() {
        assert!(
            clsx(class_toggles(&[("a", Dynamic::from(1_i64))])).is_err_and(|eval_alt_result| {
                matches!(
                    *eval_alt_result,
                    EvalAltResult::ErrorSystem(_, ref source)
                        if matches!(
                            source.downcast_ref::<RhaiComponentsError>(),
                            Some(RhaiComponentsError::ClsxValueNotBoolean { class_name, value_type })
                                if class_name == "a" && value_type == "i64"
                        )
                )
            })
        );
    }
}
