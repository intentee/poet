#[cfg(test)]
use std::ops::Range;

pub struct ResourceProviderListParams {
    pub limit: usize,
    pub offset: usize,
}

impl ResourceProviderListParams {
    #[cfg(test)]
    pub fn range(&self) -> Range<usize> {
        self.offset..(self.offset + self.limit)
    }
}
