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
