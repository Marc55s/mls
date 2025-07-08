use std::env;
use std::fs::*;
use colored::Colorize;

fn list_files(path: &str) {
    let entries = read_dir(path).unwrap_or_else(|e| panic!("Failed to read directory: {}", e));

    let paths: Vec<String> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path().display().to_string())
        .collect();
    
    for p in paths {
        if metadata(&p).unwrap().is_dir() {
            print!("{} ", p.blue());
        } else {
            print!("{} ", p.yellow());
        }
    }

    // println!("{}", paths.join("\n"));
}

fn unpack_args(args: &[String]) {
    match args.get(1).map(String::as_str) {
        Some("-l") => println!("List Arg"),
        Some("-a") => println!("All Arg"),
        Some(other) => println!("Unknown argument: {}", other),
        None => println!("No argument provided"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        unpack_args(&args);
    } else {
        list_files(".");
    }
}
