use std::fmt;

use dashmap::DashMap;

#[derive(Default)]
pub struct AnyhowErrorAggregate {
    pub errors: DashMap<String, anyhow::Error>,
}

impl fmt::Display for AnyhowErrorAggregate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            formatter,
            "Multiple errors occurred ({} total):",
            self.errors.len()
        )?;

        for entry in self.errors.iter() {
            let key = entry.key();
            let error = entry.value();

            writeln!(formatter, "\n[{}]", key)?;
            writeln!(formatter, "{:#?}", error)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;

    use super::*;

    #[test]
    fn display_lists_every_aggregated_error() {
        let aggregate = AnyhowErrorAggregate::default();

        aggregate
            .errors
            .insert("first".to_string(), anyhow!("boom one"));
        aggregate
            .errors
            .insert("second".to_string(), anyhow!("boom two"));

        let rendered = aggregate.to_string();

        assert!(rendered.contains("(2 total)"));
        assert!(rendered.contains("[first]"));
        assert!(rendered.contains("[second]"));
    }

    #[test]
    fn display_reports_zero_when_empty() {
        assert!(
            AnyhowErrorAggregate::default()
                .to_string()
                .contains("(0 total)")
        );
    }
}
