use std::{error, fs};

use clap::{
    Command,
    crate_description,
    crate_name,
    crate_version
};
use rustyline::{Editor, error::ReadlineError};

mod rx;
use crate::rx::Datasets;
use crate::rx::{cmd, io, repl};

/// Name of the file where REPL history is stored.
const HISTORY_FILE: &str = ".cone_history";

/// Code of the current program.
const APP_CODE: &str = "RX-01";

/// Builds the CLI metadata using `clap`.
fn build_cli() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(format!("{}.", crate_description!()))
}

/// Runs the main application loop and REPL interface.
fn run_app() -> Result<(), Box<dyn error::Error>> {
    // accepted REPL commands (used for hinting)
    let commands = [
        "clean", "compute", "exit", "help",
        "load", "preview", "save",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<_>>();

    let helper = repl::ReadLineHelper::new(commands);
    let mut rl = Editor::new()?;
    rl.set_helper(Some(helper));

    io::header(APP_CODE);

    if rl.load_history(HISTORY_FILE).is_err() {
        println!();
        io::print_warn(format!("no '{HISTORY_FILE}' file found"));
        let _ = fs::File::create(HISTORY_FILE)
            .map(|_| io::print_info("history file created"))
            .map_err(|_| io::print_error("failed to create history file"));
    }

    // initialize in-memory dataset collection
    let mut datasets = Datasets::new();

    loop {
        match rl.readline(&format!("\nRX {} ", io::PROMPT)) {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                match cmd::execute(trimmed, &mut datasets) {
                    Ok(true) => {}
                    Ok(false) => break,
                    Err(err) => io::print_error(format!("command error: {err}")),
                }

                rl.add_history_entry(trimmed)?;
            }
            Err(ReadlineError::Interrupted) => {
                io::print_error("process interrupted");
                io::print_error("exiting safely...");
                break;
            }
            Err(_) => {
                io::print_error("something went wrong");
                io::print_error("exiting safely...");
                break;
            }
        }
    }

    // save history upon exit
    rl.save_history(HISTORY_FILE)?;
    Ok(())
}

/// Entry point of the application.
fn main() {
    // parse command-line options (--help, --version, etc.)
    let _matches = build_cli().get_matches();

    if let Err(err) = run_app() {
        io::print_error(format!("fatal error: {err}"));
        std::process::exit(1);
    }
}
