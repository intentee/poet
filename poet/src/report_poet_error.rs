use log::error;
use poet_error_chain::error_chain::ErrorChain;

use crate::poet_error::PoetError;

pub fn report_poet_error(poet_error: PoetError) {
    error!("{}", ErrorChain { error: &poet_error });
}
