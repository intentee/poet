#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TagKind {
    Closing,
    Opening,
    SelfClosing,
}
