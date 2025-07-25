use chrono::{DateTime, Local};
use colored::ColoredString;
use colored::Colorize;
use std::fs::*;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(PartialEq)]
enum Flag {
    All,
    List,
    AllAndList,
    Empty,
}

#[derive(Debug)]
struct FileEntry {
    name: ColoredString,
    is_dir: bool,
    size: u64,
    permissions: u32,
    last_edited: Option<SystemTime>,
}

impl FileEntry {
    fn print(&self) {
        let edited_str = match self.last_edited {
            Some(time) => {
                let datetime: DateTime<Local> = DateTime::<Local>::from(time);
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            }
            None => String::from("N/A"),
        };

        println!(
            "{:<20} {:<5} {:>10} {:>4o} {}",
            self.name, self.is_dir, self.size, self.permissions, edited_str
        );
    }
}

fn get_file_entry(path: &Path) -> Result<FileEntry, io::Error> {
    let metadata = metadata(path)?;
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid File name"))?;
    let last_edited = match metadata.modified() {
        Ok(time) => Some(time),
        Err(e) => {
            eprintln!(
                "Warning: couldn't read modified time for {}: {}",
                file_name, e
            );
            None
        }
    };

    let colored_file_name = {
        if metadata.is_dir() {
            file_name.blue()
        } else {
            file_name.yellow()
        }
    };

    Ok(FileEntry {
        name: colored_file_name,
        is_dir: metadata.is_dir(),
        size: metadata.len(),
        permissions: metadata.permissions().mode() & 0o777,
        last_edited,
    })
}

fn is_dotfile(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name_str| name_str.starts_with('.'))
        .unwrap_or(false)
}

fn get_files_from_path(path: &Path) -> Vec<PathBuf> {
    read_dir(path)
        .unwrap_or_else(|e| panic!("Failed to read directory {}: {}", path.display(), e))
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect()
}

fn filter_files(path: &Path, flag: &Flag) -> Vec<PathBuf> {
    let files = get_files_from_path(path);

    // filter dotfiles
    let mut filtered = match flag {
        Flag::All => files,
        _ => files.into_iter().filter(|file| !is_dotfile(file)).collect(),
    };

    // sort files by dir and no dir
    filtered.sort_by_key(|path| metadata(path).map(|m| !m.is_dir()).unwrap_or(true));
    filtered
}

fn print_colorized_strings(file_names: &mut [PathBuf]) {
    // Sort directories first
    for path in file_names.iter() {
        let display_name = path.display().to_string();
        let colored_name = if metadata(path).map(|m| m.is_dir()).unwrap_or(false) {
            display_name.blue()
        } else {
            display_name.yellow()
        };
        println!("{}", colored_name);
    }
}

fn unpack_args(args: Args) -> (PathBuf, Flag) {
    let result = match (args.all, args.long) {
        (true, true) => Flag::AllAndList,
        (true, false) => Flag::All,
        (false, true) => Flag::List,
        (_, _) => Flag::Empty,
    };

    (args.path, result)
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Show hidden files
    #[arg(short = 'a', long)]
    all: bool,

    /// Use long listing format
    #[arg(short = 'l', long)]
    long: bool,

    /// Directory to list (optional, defaults to current)
    #[arg(default_value = ".")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let (path, flag) = unpack_args(args);
    let _files = get_files_from_path(path.as_path());
    let mut filtered_files = filter_files(path.as_path(), &flag);

    if flag == Flag::List || flag == Flag::AllAndList {
        for file in filtered_files {
            match get_file_entry(&file) {
                Ok(entry) => entry.print(),
                Err(e) => println!("Listing detailed file information failed: {}", e),
            };
        }
    } else {
        print_colorized_strings(&mut filtered_files);
    }
}
