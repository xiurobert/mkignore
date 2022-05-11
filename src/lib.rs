use std::fs::{File};
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
    let mut formats: Vec<Format> = Vec::new();
    for thing in stuff {
        match thing.as_str() {
            "jb" => {
                println!(" - Will process [jetbrains]");
                formats.push(Format::JB);
            }
            "py" => {
                println!(" - Will process [python]");
                formats.push(Format::Py);
            }
            _ => {}
        }
    }
    put_in_gitignore(formats);
    Ok(())
}

enum Format {
    JB,
    Py,
}

fn build_ignore(formats: Vec<Format>) -> String {
    let mut ignore_string = String::new();
    for format in formats {
        match format {
            Format::JB => {
                if let Some(jb_ignore) = read_in("ignore-texts/jb.gitignore", false) {
                    ignore_string.push_str(&jb_ignore);
                } else {
                    eprintln!("<F> Failed to read in jb.gitignore");
                }
            },
            Format::Py => {
                if let Some(py_ignore) = read_in("ignore-texts/py.gitignore", false) {
                    ignore_string.push_str(&py_ignore);
                } else {
                    eprintln!("<F> Failed to read in py.gitignore");
                }
            },
        }
        ignore_string.push('\n');

    }
    ignore_string
}


fn put_in_gitignore(formats: Vec<Format>) {

    let ignore_string = build_ignore(formats);

    let ignore_file = File::options()
        .read(true)
        .append(true)
        .create(true)
        .open("test.gitignore");
    if let Ok(f_ignore) = ignore_file {
        match append_to_file(f_ignore, &ignore_string) {
            Ok(bytes_written) => {
                println!("Wrote {} bytes", bytes_written);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    } else {
        eprintln!("Couldn't open the ignore file for modification");
    }

}

fn append_to_file(mut file: File, stuff: &str) -> Result<usize, String> {
    let mut new_contents = String::new();
    new_contents.push_str(stuff);
    new_contents.push('\n');

    if let Ok(bytes_written) = file.write(new_contents.as_bytes()) {
        return Ok(bytes_written)
    }

    Err(String::from("Could not append to the file"))
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