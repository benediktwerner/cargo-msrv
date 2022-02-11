mod list;
mod show;
mod verify;

use crate::cli_v3::list::ListSubCommand;
use crate::cli_v3::show::ShowSubCommand;
use crate::cli_v3::verify::VerifySubCommand;
use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    List(ListSubCommand),
    Show(ShowSubCommand),
    Verify(VerifySubCommand),
}
