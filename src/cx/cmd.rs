use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use shlex::split;

use crate::io;


/// CLI commands supported by the REPL.
#[derive(Subcommand, Debug)]
enum Commands {
    /// Load a file or directory.
    Load(LoadCmd),
    /// Preview loaded data.
    Preview(PreviewCmd),
    /// Filter results based on a pattern.
    Filter(FilterCmd),
    /// Exit the REPL.
    Exit,
}

/// CONE: Top-level parser for REPL commands.
#[derive(Parser, Debug)]
#[command(multicall = true, disable_help_flag = true)]
struct ReplCli {
    /// The entered subcommand.
    #[command(subcommand)]
    command: Commands,
}


/// Parse a line of input and execute the corresponding command.
///
/// Splits the input as shell tokens, parses it into `ReplCli`, and dispatches to handlers.
pub fn parse_input_line(line: &str) {
    // try splitting input into shell-like tokens
    if let Some(args) = split(line) {
        // attempt to parse tokens as our CLI
        match ReplCli::try_parse_from(args) {
            Ok(cli) => {
                // Tenemos un ReplCli parseado correctamente
                match cli.command {
                    Commands::Load(_) => {
                        io::print_info("You choosed the LOAD command.")
                     },
                    Commands::Preview(_) => {
                        io::print_info("You choosed the PREVIEW command.")
                     },
                    Commands::Filter(_) => {
                        io::print_info("You choosed the FILTER command.")
                     },
                    Commands::Exit => {
                        io::print_info("bye bye")
                     },
                }
            },
            Err(err) => {
                // print clap-generated error or help message
                let _ = err.print();
            }
        }
    }
}

/// Arguments for the `load` subcommand.
#[derive(Args, Debug)]
struct LoadCmd {
    /// Path of file to load.
    #[arg(
        short,
        long,
        value_name = "FILE",
    )]
    file: Option<PathBuf>,

    /// Path of directory to load.
    #[arg(
        short,
        long,
        value_name = "DIR",
    )]
    dir: Option<PathBuf>,
}

/// Arguments for the `preview` subcommand.
///
/// Customize preview options such as number of lines (future extension).
#[derive(Args, Debug)]
struct PreviewCmd {
    // example: number of lines to preview
    // #[arg(short = 'n', long, default_value = "10", help = "number of lines to show")]
    // pub lines: usize,
}

/// Arguments for the `filter` subcommand.
#[derive(Args, Debug)]
struct FilterCmd {
    /// Pattern to filter by.
    #[arg(value_name = "PATTERN", help = "pattern to filter by")]
    pattern: String,

    /// filter without case sensitivity.
    #[arg(short = 'i', long, help = "filter without case sensitivity")]
    ignore_case: bool,
}