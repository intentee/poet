use std::error::Error;
use std::fmt;
use std::iter::successors;

fn error_source<'error>(
    error: &&'error (dyn Error + 'static),
) -> Option<&'error (dyn Error + 'static)> {
    (*error).source()
}

pub struct ErrorChain<'error> {
    pub error: &'error (dyn Error + 'static),
}

impl<'error> ErrorChain<'error> {
    pub fn causes(&self) -> impl Iterator<Item = &'error (dyn Error + 'static)> {
        successors(Some(self.error), error_source)
    }
}

impl fmt::Display for ErrorChain<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut causes = self.causes();

        causes
            .next()
            .map_or(Ok(()), |error| write!(formatter, "{error}"))
            .and_then(|()| causes.try_for_each(|cause| write!(formatter, ": {cause}")))
    }
}
