mod parser;
mod instrument;
mod engine;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Editor, Result};
use crate::parser::parser::Parser;

fn main() -> Result<()> {

let mut rl = DefaultEditor::new()?;



loop{

    let readline = rl.readline(">>");
    match readline {
        Ok(line) => {
            let input = line.trim();
            Parser::readCommand(input);
        }
        Err(ReadlineError::Interrupted) => {
            println!("Muse is Interrupted");
            break;
        }
        Err(err) => {
            println!("Error: {:?}", err);
            break;
        }
    }

}



    Ok(())

}
