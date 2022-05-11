use std::process::exit;
use mkignore::{parse_args, put_stuff};

fn main() {
    println!("mkignore v0.0.1");
    match parse_args(std::env::args()) {
        Ok(args) => {
            let placed = put_stuff(args);
            match placed {
                Ok(_) => {
                    println!("Successfully placed files");
                    exit(0);
                }
                Err(e) => {
                    println!("{}", e);
                    exit(1);
                }
            }
        }
        Err(e) => {
            println!("{}", e);
            exit(1)
        }
    }

}
