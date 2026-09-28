use std::sync::LazyLock;

use syntect::parsing::SyntaxSet;

pub static DEFAULT_SYNTAX_SET: LazyLock<SyntaxSet> =
    LazyLock::new(SyntaxSet::load_defaults_newlines);
