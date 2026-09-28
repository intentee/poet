use std::error::Error;
use std::fmt;
use std::iter::successors;

fn error_source<'error>(
    error: &&'error (dyn Error + 'static),
) -> Option<&'error (dyn Error + 'static)> {
    (*error).source()
}

#[derive(Debug)]
pub struct DocumentError<TError> {
    pub basename: String,
    pub error: TError,
}

impl<TError: Error + 'static> fmt::Display for DocumentError<TError> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "{}:", self.basename).and_then(|()| {
            successors(Some(&self.error as &(dyn Error + 'static)), error_source)
                .try_for_each(|cause| writeln!(formatter, "- {cause}"))
        })
    }
}
