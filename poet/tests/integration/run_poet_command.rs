use std::iter::once;

use clap::Parser as _;
use poet::cli::Cli;
use poet::poet_error::PoetError;

use crate::poet_tests_error::PoetTestsError;

pub async fn run_poet_command(arguments: &[&str]) -> Result<Result<(), PoetError>, PoetTestsError> {
    Ok(Cli::try_parse_from(once(&"poet").chain(arguments))?
        .command
        .into_handler()
        .handle()
        .await)
}
