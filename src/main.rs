use std::io::{stdin, stdout, Write};
use std::process::Command;

fn main(){
    loop {
        // use the `>` as the prompt
        print!("> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        let mut child = Command::new(command)
            .spawn()
            .unwrap();

        // don't accept another command until this one completes
        child.wait(); 
    }
}