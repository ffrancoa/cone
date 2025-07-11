use std::path::PathBuf;

use clap::{Args, Error, Parser, Subcommand};
use polars::prelude::DataFrame;
use shlex::split;

use crate::rx::io;


/// CLI commands supported by the REPL.
#[derive(Subcommand, Debug)]
enum Commands {
    /// Load a file or directory.
    Load(LoadCmd),
    /// Preview loaded data.
    Preview(PreviewCmd),
    /// Filter results based on a pattern.
    Filter(FilterCmd),
    /// Exit this program.
    Exit,
}

/// CONE .:. Read-Eval-Print-Loop 
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
pub fn execute(line: &str, _dataset: &mut DataFrame) -> Result<bool, Error> {
    // try splitting input into shell-like tokens
    if let Some(args) = split(line) {
        // attempt to parse tokens as our CLI
        match ReplCli::try_parse_from(args) {
            Ok(cli) => {
                match cli.command {
                    Commands::Exit => {
                        io::print_info("exiting...");
                        io::print_info("goodbye!");
                        return Ok(false)
                    },
                    Commands::Load(args) => {
                        if let Some(file) = &args.file {
                            io::print_info(format!("loading file: {:?}", file));
                        } else if let Some(dir) = &args.dir {
                            io::print_info(format!("loading directory: {:?}", dir));
                        } else {
                            io::print_error("no file or directory provided.");
                        }
                    },
                    Commands::Preview(_) => {
                        io::print_info("You chose the PREVIEW command.")
                    },
                    Commands::Filter(_) => {
                        io::print_info("You choosed the FILTER command.")
                    },
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