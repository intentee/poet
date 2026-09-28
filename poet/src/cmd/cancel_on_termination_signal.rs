use tokio_util::sync::CancellationToken;

use crate::poet_error::PoetError;

pub fn cancel_on_termination_signal(
    cancellation_token: CancellationToken,
) -> Result<(), PoetError> {
    ctrlc::set_handler(move || {
        cancellation_token.cancel();
    })
    .map_err(PoetError::SetCtrlcHandler)
}
