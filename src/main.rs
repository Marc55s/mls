use colored::Colorize;
use std::env;
use std::fmt::Debug;
use std::fs::*;
// use std::os::unix::fs::PermissionsExt;

#[derive(PartialEq)]
enum Flags {
    All,
    List,
    Empty,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum FileType {
    File,
    Dir,
}

struct ColorText {
    text: String,
    file_type: FileType,
}

impl ColorText {
    fn print_color(&self) {
        if self.file_type == FileType::Dir {
            print!("{} ", self.text.blue())
        } else {
            print!("{} ", self.text.yellow())
        }
    }
}

fn is_dotfile(file_name: &str) -> bool {
    file_name.starts_with('.')
}

fn get_files_from_path(path: &str) -> Vec<String> {
    let entries = read_dir(path).unwrap_or_else(|e| panic!("Failed to read directory: {}", e));

    let paths: Vec<String> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path().display().to_string())
        .map(|s| s.replace("./", ""))
        .collect();

    paths
}

fn list_files(path: &str, flag: Flags) {
    let mut dir_names: Vec<ColorText> = Vec::new();
    let files = get_files_from_path(path);

    for p in files {
        if is_dotfile(&p) && flag != Flags::All {
            continue;
        }
        println!("{:?}", metadata(&p).unwrap().permissions().readonly());
        println!("{:?}", metadata(&p).unwrap().permissions());
        println!("{:?}", metadata(&p).unwrap().accessed().unwrap());
        if metadata(&p).unwrap().is_dir() {
            let ct = ColorText {
                text: p,
                file_type: FileType::Dir,
            };
            dir_names.push(ct);
        } else {
            let ct = ColorText {
                text: p.into(),
                file_type: FileType::File,
            };
            dir_names.push(ct);
        }
    }

    dir_names.sort_by(|a, b| a.file_type.cmp(&b.file_type));
    dir_names.iter().for_each(|e| e.print_color());
}

fn unpack_args(args: &Vec<String>) {
    match args.get(1).map(String::as_str) {
        Some("-l") => println!("List Arg"),
        Some("-a") => list_files(".", Flags::All),
        Some(other) => println!("Unknown argument: {}", other),
        None => println!("No argument provided"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        unpack_args(&args);
    } else {
        list_files(".", Flags::Empty);
    }
}
