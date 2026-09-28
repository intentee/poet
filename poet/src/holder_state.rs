#[derive(Clone)]
pub enum HolderState<TItem> {
    NotReady,
    Ready(TItem),
}

impl<TItem> HolderState<TItem> {
    pub fn ready_or<TError>(self, not_ready_error: TError) -> Result<TItem, TError> {
        match self {
            Self::NotReady => Err(not_ready_error),
            Self::Ready(item) => Ok(item),
        }
    }
}
