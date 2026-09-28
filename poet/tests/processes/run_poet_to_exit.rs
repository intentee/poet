use std::process::ExitStatus;

use crate::poet_process_tests_error::PoetProcessTestsError;
use crate::spawn_poet::spawn_poet;

pub fn run_poet_to_exit(arguments: &[&str]) -> Result<ExitStatus, PoetProcessTestsError> {
    Ok(spawn_poet(arguments)?.wait()?)
}
