use std::{error, fs};

use clap::{
    Command,
    crate_description,
    crate_name,
    crate_version};
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

/// Run the main application logic.
fn run_app() -> Result<(), Box<dyn error::Error>> {
    // supported commands for the REPL
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

    // load or create history file if it doesn't exists
    if rl.load_history(HISTORY_FILE).is_err() {
        println!();
        io::print_warn("No previous history. Creating a new one...");
        fs::File::create(HISTORY_FILE)
            .map(|_| io::print_info("History file created successfully."))
            .map_err(|_| io::print_error("History file cannot be created."))
            .ok();
    }

    // main REPL loop
    loop {
        match rl.readline("CX ❯ ") {
            Ok(buffer) => {
                let trimmed_line = buffer.trim();
                if trimmed_line.is_empty() {
                    continue
                }
                rl.add_history_entry(trimmed_line)?;

                cmd::parse_input_line(trimmed_line);

            }
            Err(ReadlineError::Interrupted) => {
                io::print_error("Process interrupted. Exiting safely...");
                break
            }
            Err(err) => {
                io::print_error(format!("Something went wrong ({}).", err));
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
