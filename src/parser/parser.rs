use crate::parser::command::Command;

pub struct Parser;


impl Parser {
   pub fn readCommand(command: &str){
        let tokens: Vec<&str> = command.split_whitespace().collect();
        println!("Tokens: {:?}", tokens);
    }
}