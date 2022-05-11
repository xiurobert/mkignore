use std::fs::File;
use std::io::{Read, Write};

pub fn parse_args(args: std::env::Args) -> Result<Vec<String>, String> {
    let acceptable_formats = vec![
        "jb",
        "py"
    ];

    let args = args.skip(1);
    let formats: Vec<String> = args.collect();

    if formats.is_empty() {
        return Err("No formats specified".to_string());
    }

    for format in &formats {
        if !acceptable_formats.contains(&format.as_str()) {
            return Err(format!("{} is not a valid format", format));
        }
    }
    Ok(formats)
}


pub fn put_stuff(stuff: Vec<String>) -> Result<(), String> {
    if stuff.is_empty() {
        return Err("No stuff specified".to_string());
    }
    for thing in stuff {
        match thing.as_str() {
            "jb" => {
                println!("Processing jetbrains");
                put_jetbrains_in_gitignore();
            }
            "py" => {
                println!("Processing python");
            }
            _ => {}
        }
    }
    Ok(())
}



fn put_jetbrains_in_gitignore() {
    let jb_gitignore_file = File::options().
        read(true).
        open("ignore-texts/jb.mkignore");

    if let Ok(..) = jb_gitignore_file {
        let mut buf = String::new();
        if let Ok(r) = jb_gitignore_file.unwrap().read_to_string(&mut buf) {
            println!("Read {} bytes from jb.mkignore", r);
            let maybe_gitignore_file = File::options()
                .read(true)
                .write(true)
                .create(true)
                .open(".gitignore");
            if let Ok(mut gitignore_file) = maybe_gitignore_file {
                let mut current_contents = String::new();
                gitignore_file.read_to_string(&mut current_contents);
                let maybe_written = gitignore_file.write(buf.as_bytes());
                if let Ok(b) =  maybe_written {
                    println!("Ok, written {} bytes", b);
                }
            } else {
                eprintln!("Something went wrong trying to create/open .gitignore");
            }
        }

    }




}