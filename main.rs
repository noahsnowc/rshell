use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio}; 

fn main(){
    loop{
        print!("> ");
        stdout().flush();


        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();

        let args = parts;
        match command{
            "cd" =>{
                //default to '/' as new directory if one was not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            "exit" => return,
            command =>{
                let mut child = Command::new(command)
                .args(args)
                .spawn();
    
                //don't accept another command until this one completes.
                match child {
                    Ok(mut child) => {child.wait();},
                    Err(e) => eprintln!("{}", e),
                };
            }
            
        }
        
    }
      
}