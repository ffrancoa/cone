use std::{error, fs};

use clap::Command;
use rustyline::error::ReadlineError;
use rustyline::Editor;

mod cx;
use crate::cx::{io, repl};

const HISTORY_FILE: &str = ".cone_history";

/// Run the main application logic.
fn run_app() -> Result<(), Box<dyn error::Error>> {
    // supported commands for the REPL
    let commands = [
        "CLEAN", "COMPUTE", "EXIT", "HELP",
        "LIST", "LOAD", "PREVIEW", "PROCESS",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    // initialize line editor with helper
    let helper = repl::ReadLineHelper::new(commands.clone());
    let mut rl = Editor::new()?;
    rl.set_helper(Some(helper));

    // load or create history file if it doesn't exists
    if rl.load_history(HISTORY_FILE).is_err() {
        io::print_warn("No previous history. Creating a new one...");
        fs::File::create(HISTORY_FILE)?;
    }

    // main REPL loop
    loop {
        match rl.readline("CX ❯ ") {
            Ok(input) => {
                let entry = input.trim();
                if entry.is_empty() {
                    continue
                }
                rl.add_history_entry(entry)?;

                let args: Vec<&str> = entry.split_whitespace().collect();
                let cmd = args[0].to_ascii_uppercase();

                if commands.contains(&cmd) {
                    io::print_info(format!("You've entered the '{}' command.", cmd));
                } else {
                    io::print_error(format!("Command '{}' does not exist.", cmd));
                }
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

    // save history on exit
    rl.save_history(HISTORY_FILE)?;
    Ok(())
}

fn main() {
    let _ = Command::new(clap::crate_name!())
        .version(clap::crate_version!()) // usa la versión del Cargo.toml automáticamente
        .about("CONE: CPTu Operations and Numerical Exploration.") // descripción breve del programa
        .get_matches();

    if let Err(err) = run_app() {
        io::print_error(format!("Application error: {}.", err));
        std::process::exit(1);
    }
}
