use rustyline::DefaultEditor;

mod cx;

use crate::cx::io;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut rl = DefaultEditor::new()?;

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
                let cmd = args[0].to_ascii_uppercase();
                match cmd.as_str() {
                    "LOAD" => {
                        io::print_info("You've entered the LOAD command.")
                    }
                    _ => {
                        io::print_error(format!("Command {} does not exist.", cmd))
                    }
                }
            }
            Err(_) => {
                io::print_error("Oops... Something went wrong.");
                break
            }
        }
    }

    Ok(())
}
