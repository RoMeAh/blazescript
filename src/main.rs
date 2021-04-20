use blazescript::{core::interpreter::interpreter::Interpreter, Interpret};
use rustyline::{error::ReadlineError, Editor};

fn main() {
    let mut rl = Editor::<()>::new();
    println!("Blazescript REPL.");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let result =
                    Interpreter::from_source("REPL", Box::leak(line.to_owned().into_boxed_str()));
                match result {
                    Ok(n) => println!("{}", n),
                    Err(e) => println!("{}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
