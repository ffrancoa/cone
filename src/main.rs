use std::{error, fs};

use clap::{
    Command,
    crate_description,
    crate_name,
    crate_version};
use polars::prelude::DataFrame;
use rustyline::{
    Editor,
    error::ReadlineError,
};

mod cx;
use crate::cx::{cmd, io, repl};

/// Name of the file where REPL history is stored.
const HISTORY_FILE: &str = ".cone_history";
/// Code of the current program.
const APP_CODE: &str = "CX-01";


/// Build the CLI using `clap`.
fn build_cli() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(format!("{}.", crate_description!()))
}

/// Runs the main application loop and REPL interface.
fn run_app() -> Result<(), Box<dyn error::Error>> {
    // list of accepted REPL commands for hinting and highlighting
    let commands = [
        "clean", "compute", "exit", "help",
        "load", "preview", "save",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    // initialize line editor with helper
    let helper = repl::ReadLineHelper::new(commands);
    let mut rl = Editor::new()?;
    rl.set_helper(Some(helper));

    // print header when the REPL starts
    io::header(APP_CODE);

    // load history file or create it if it does not exist
    if rl.load_history(HISTORY_FILE).is_err() {
        println!();
        io::print_warn(format!("no '{}' file in current directory", HISTORY_FILE));
        let _ = fs::File::create(HISTORY_FILE)
            .map(|_| io::print_info("history file created"))
            .map_err(|_| io::print_error("history file cannot be created"));
    }

    // allocate an empty dataset
    let mut data = DataFrame::empty();

    // main REPL loop
    loop {
        match rl.readline("\nCX ❯ ") {
            Ok(buffer) => {
                let trimmed_line = buffer.trim();

                if trimmed_line.is_empty() {
                    continue
                }

                if let Ok(false) = cmd::execute(trimmed_line, &mut data) {
                    break;
                }

                rl.add_history_entry(trimmed_line)?;
            }
            Err(ReadlineError::Interrupted) => {
                io::print_error("process interrupted");
                io::print_error("exiting safely...");
                break
            }
            Err(err) => {
                io::print_error(format!("something went wrong ({}).", err));
                break
            }
        }
    }

    // save history upon exit
    rl.save_history(HISTORY_FILE)?;
    Ok(())
}

fn main() {
    // parse command-line options (currently only --help, --version)
    let _matches = build_cli().get_matches();

    if let Err(err) = run_app() {
        io::print_error(format!("Application error: {}", err));
        std::process::exit(1);
    }
}
