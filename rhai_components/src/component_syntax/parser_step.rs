use super::output_symbol::OutputSymbol;
use super::parser_state::ParserState;

pub enum ParserStep {
    Advance {
        next_state: ParserState,
        next_symbol: &'static str,
    },
    Begin,
    Emit {
        output_symbol: OutputSymbol,
        next_state: ParserState,
        next_symbol: &'static str,
    },
    Finish,
}
