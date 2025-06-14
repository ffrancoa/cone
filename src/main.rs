use rustyline::Editor;

mod cx;

use crate::cx::io;
use crate::cx::repl;

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

                if commands.contains(&cmd) {
                     io::print_info(format!("You've entered the {} command", cmd))
                } else {
                    io::print_error(format!("Command {} does not exist.", cmd))
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
