use colored::Colorize;
use std::env;
use std::fs::*;
// use std::os::unix::fs::PermissionsExt;

#[derive(PartialEq)]
enum Flag {
    All,
    List,
    Empty,
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

fn filter_files(path: &str, flags: Flag) -> Vec<String> {
    let files = get_files_from_path(path);

    let filtered_files: Vec<String> = match flags {
        Flag::List => files
            .iter()
            .filter(|file| !is_dotfile(file))
            .cloned()
            .collect(),
        Flag::Empty => files
            .iter()
            .filter(|file| !is_dotfile(file))
            .cloned()
            .collect(),
        Flag::All => files,
    };

    filtered_files
}

fn print_colorized_strings(file_names: &mut [String]) {
    file_names.sort_by_key(|path| {
        // If metadata fails, treat it as a file (false)
        metadata(path).map(|m| !m.is_dir()).unwrap_or(true)
    });

    file_names.iter().map(|s| {
        if metadata(s).unwrap().is_dir() {
            s.blue().to_string()
        }else {
            s.yellow().to_string()
        }
    }).for_each(|p| println!("{}", p));
}

fn unpack_args(args: &[String]) -> (&str, Flag) {
    let flag: Flag = match args.get(1).map(String::as_str) {
        Some("-l") => Flag::List,
        Some("-a") => Flag::All,
        Some(_) => Flag::Empty,
        None => Flag::Empty,
    };
    (".", flag)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Received {} args", args.len());

    if args.len() <= 2 {
        let (flag, path) = unpack_args(&args);
        let mut filtered = filter_files(flag, path);
        print_colorized_strings(&mut filtered);
    }
}
