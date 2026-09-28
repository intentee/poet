use poet_filesystem::storage::Storage;
use rhai::Engine;

pub struct CompileShortcodesParams<'params> {
    pub register_rhai_types: fn(&mut Engine),
    pub source_filesystem: &'params Storage,
}
