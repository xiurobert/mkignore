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
                println!(" - Processing [jetbrains]");
                put_jetbrains_in_gitignore();
            }
            "py" => {
                println!(" - Processing [python]");
            }
            _ => {}
        }
    }
    Ok(())
}



fn put_jetbrains_in_gitignore() {
    if let Some(jb_ignore) = read_in("ignore-texts/jb.mkignore", false) {
        let ignore_file = File::options().read(true).write(true).create(true).open(".gitignore");
        if let Ok(mut f_ignore) = ignore_file {
            match append_to_file(f_ignore, &jb_ignore) {
                Ok(bytes_written) => {
                    println!("Wrote {} bytes", bytes_written);
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        } else {
            eprintln!("Couldn't write")
        }
    } else {
        eprintln!("The jb.mkignore file was empty");
    }

}

fn append_to_file(mut file: File, stuff: &str) -> Result<usize, String> {
    if let Some(curr_contents) = read_file(&file) {
        let mut new_contents = curr_contents;
        new_contents.push_str(stuff);
        new_contents.push('\n');

        if let Ok(bytes_written) = file.write(new_contents.as_bytes()) {
            return Ok(bytes_written)
        }

        return Err(String::from("Could not append to the file"));
    }
    Err(String::from("The file "))
}


fn read_file(mut file: &File) -> Option<String> {
    let mut buffer = String::new();
    if let Ok(..) = file.read_to_string(&mut buffer) {
        return Some(buffer);
    }
    None
}

fn read_in(file_path: &str, try_make: bool) -> Option<String> {
    let the_file = File::options()
        .read(true)
        .create(try_make)
        .open(file_path);

    if let Ok(f) = the_file {
        return read_file(&f);
    }
    None
}