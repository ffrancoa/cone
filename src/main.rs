use rustyline::Editor;

mod cx;

use crate::cx::io;
use crate::cx::repl;

const HISTORY_FILENAME: &str = ".cone_history";

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let commands: Vec<String> = vec![
        "CLEAN".into(),
        "COMPUTE".into(),
        "EXIT".into(),
        "HELP".into(),
        "LIST".into(),
        "LOAD".into(),
        "PREVIEW".into(),
        "PROCESS".into(),
    ];

    let helper = repl::ReadLineHelper::new(commands.clone());

    let mut rl = Editor::new()?;
    rl.set_helper(Some(helper));

    if rl.load_history(HISTORY_FILENAME).is_err() {
        cx::io::print_error("No previous history. Creating a new one...");
        std::fs::File::create(HISTORY_FILENAME)?;
    }

    loop {
        let raw_input = rl.readline("CX ❯ ");

        match raw_input {
            Ok(raw_input) => {
                let args: Vec<&str> = raw_input
                    .split_whitespace()
                    .collect();
                
                if args.is_empty() {
                    continue
                }

                rl.add_history_entry(raw_input.trim())?;
                println!("{}", raw_input.trim());

                let cmd = args[0].to_ascii_uppercase();

                if commands.contains(&cmd) {
                     io::print_info(format!("You've entered the '{}' command.", cmd))
                } else {
                    io::print_error(format!("command '{}' does not exist", cmd))
                }
            }
            Err(_) => {
                io::print_error("Process interrupted. Exiting safely...");
                break
            }
        }
    }

    rl.save_history(HISTORY_FILENAME)?;
    Ok(())
}
