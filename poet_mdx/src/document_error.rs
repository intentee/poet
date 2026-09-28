use std::error::Error;
use std::fmt;

use poet_error_chain::error_chain::ErrorChain;

#[derive(Debug)]
pub struct DocumentError<TError> {
    pub basename: String,
    pub error: TError,
}

impl<TError: Error + 'static> fmt::Display for DocumentError<TError> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "{}:", self.basename).and_then(|()| {
            ErrorChain { error: &self.error }
                .causes()
                .try_for_each(|cause| writeln!(formatter, "- {cause}"))
        })
    }
}
