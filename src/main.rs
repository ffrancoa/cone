use rustyline::DefaultEditor;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut rl = DefaultEditor::new()?;

    loop {
        let input = rl.readline("CX ❯ ");

        match input {
            Ok(_) => println!("Everything's good."),
            Err(_) => {
                println!("Oops... Something went wrong.");
                break
            }
        }
    }

    Ok(())
}
