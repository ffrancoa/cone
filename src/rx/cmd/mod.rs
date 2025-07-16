pub mod load;
pub mod preview;
pub mod save;

use clap::{Error, Parser, Subcommand};
use shlex::split;

use crate::rx::io;
use crate::rx::Datasets;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Load a file or directory.
    Load(load::LoadCmd),
    /// Preview loaded data.
    Preview(preview::PreviewCmd),
    /// Save current data.
    Save(save::SaveCmd),
    /// Exit the REPL.
    Exit,
}

/// CONE .:. Read-Eval-Print-Loop <REPL>
#[derive(Parser, Debug)]
#[command(multicall = true, disable_help_flag = true)]
struct ReplCli {
    #[command(subcommand)]
    command: Commands,
}

/// Parses a line of input and executes the corresponding command.
///
/// Splits the input as shell tokens, parses it into `ReplCli`, and dispatches to handlers.
pub fn execute(line: &str, datasets: &mut Datasets) -> Result<bool, Error> {
    // try splitting input into shell-like tokens
    if let Some(args) = split(line) {
        match ReplCli::try_parse_from(args) {
            Ok(cli) => {
                match cli.command {
                    Commands::Exit => {
                        io::print_info("exiting...");
                        io::print_info("goodbye!");
                        return Ok(false)
                    },
                    Commands::Load(cmd) => return load::run(cmd, datasets),
                    Commands::Preview(cmd) => preview::run(cmd, datasets),
                    Commands::Save(cmd) => save::run(cmd, datasets),
                }
            },
            Err(err) => {
                // print clap-generated error or help message
                let _ = err.print();
            }
        }
    }
    Ok(true)
}
