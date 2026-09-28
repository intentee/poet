use clap::Parser;

use crate::cli_command::CliCommand;

#[derive(Parser)]
#[command(arg_required_else_help(true), version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,
}
