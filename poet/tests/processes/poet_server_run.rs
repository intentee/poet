use std::process::ExitStatus;

pub struct PoetServerRun<TOutcome> {
    pub exit_status: ExitStatus,
    pub outcome: TOutcome,
}
