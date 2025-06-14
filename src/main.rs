use rustyline::DefaultEditor;

mod cx;

use crate::cx::io;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut rl = DefaultEditor::new()?;

    loop {
        let inputline = rl.readline("CX ❯ ");

        match inputline {
            Ok(_) => {
                io::print_info("Everything's good.")
            }
            Err(_) => {
                io::print_error("Oops... Something went wrong.");
                break
            }
        }
    }

    Ok(())
}
