use std::net::SocketAddr;
use std::net::TcpStream;

use nix::sys::signal::Signal;
use nix::sys::signal::kill;
use nix::unistd::Pid;

use crate::poet_process_tests_error::PoetProcessTestsError;
use crate::poet_server_run::PoetServerRun;
use crate::spawn_poet::spawn_poet;

pub fn run_poet_server<TOutcome>(
    arguments: &[&str],
    address: SocketAddr,
    interaction: impl FnOnce() -> TOutcome,
) -> Result<PoetServerRun<TOutcome>, PoetProcessTestsError> {
    let mut poet_child = spawn_poet(arguments)?;

    while TcpStream::connect(address).is_err() {
        if let Some(exit_status) = poet_child.try_wait()? {
            return Err(PoetProcessTestsError::ProcessExitedBeforeListening { exit_status });
        }
    }

    let outcome = interaction();

    kill(
        Pid::from_raw(i32::try_from(poet_child.id())?),
        Signal::SIGINT,
    )?;

    Ok(PoetServerRun {
        exit_status: poet_child.wait()?,
        outcome,
    })
}
