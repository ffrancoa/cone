use std::fs;
use std::path::{Path, PathBuf};

use clap::{ArgGroup, Args};
use polars::prelude::DataFrame;

use crate::rx::io;
use crate::rx::Datasets;

/// Arguments for the `load` subcommand.
#[derive(Args, Debug)]
#[command(group(
    ArgGroup::new("load_flags")
        .args(["files", "dir"])
        .required(true)
))]
pub struct LoadCmd {
    /// Paths of files to load.
    #[arg(short, long, value_name = "FILE", num_args = 1..)]
    files: Vec<PathBuf>,

    /// Path of directory to load.
    #[arg(short, long, value_name = "DIR")]
    dir: Option<PathBuf>,
}

/// Paths of validated targets for the `load` command.
#[derive(Debug)]
pub struct LoadTargets {
    pub files: Vec<PathBuf>,           // multiple file paths
    pub files_from_dir: Vec<PathBuf>,  // files from a directory
}

/// Executes the `load` command by validating and importing a file or directory.
pub fn run(cmd: LoadCmd, _datasets: &mut Datasets) -> Result<bool, clap::Error> {
    let mut targets = LoadTargets {
        files: Vec::new(),
        files_from_dir: Vec::new(),
    };

    // validate and collect valid files from `-f`
    for path in &cmd.files {
        if let Some(file_path) = validate_file_path(path) {
            let _df = DataFrame::empty(); // TODO: real file reading

            targets.files.push(file_path);
        }
        todo!("load file(s)")
    }

    // validate and collect valid files from `-d`
    if let Some(dir_path) = &cmd.dir {
        let files_paths = validate_dir_path(dir_path);
        if !files_paths.is_empty() {
            targets.files_from_dir = files_paths;
        }
        todo!("load files from directory")
    }

    Ok(true)
}

/// Prompts the user to assign a name to the dataset.
/// 
/// Returns the user input or a default name if input is empty.
fn ask_dataset_name(path: &Path, datasets: &Datasets) -> String {
    let default_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("dataset")
        .to_string();

    loop {
        io::input_prompt("Enter the name");

        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("⚠ Failed to read input. Try again.");
            continue;
        }

        let name = input.trim();
        let final_name = if name.is_empty() {
            default_name.clone()
        } else {
            name.to_string()
        };

        if datasets.contains_key(&final_name) {
            println!("⚠ Name '{}' already exists. Choose a different name.", final_name);
            continue;
        }

        return final_name;
    }
}

/// Validates the path to a single file and prints errors if it is invalid.
fn validate_file_path(path: &Path) -> Option<PathBuf> {
    if !path.exists() {
        io::print_error(format!("'{}' does not exist.", path.display()));
        return None;
    }

    if !path.is_file() {
        io::print_error(format!("'{}' is not a file.", path.display()));
        return None;
    }

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase);

    match ext.as_deref() {
        Some("csv") | Some("xlsx") => {
            io::print_info(format!("Valid file: {}", path.display()));
            Some(path.to_path_buf())
        }
        _ => {
            io::print_error(format!(
                "'{}' has an unsupported file extension (.csv or .xlsx expected).",
                path.display()
            ));
            None
        }
    }
}

/// Validates a directory path and checks for CSV or XLSX files.
fn validate_dir_path(path: &PathBuf) -> Vec<PathBuf> {
    if !path.exists() {
        io::print_error(format!("'{}' does not exist.", path.display()));
        return Vec::new();
    }

    if !path.is_dir() {
        io::print_error(format!("'{}' is not a directory.", path.display()));
        return Vec::new();
    }

    // search for at least one valid file
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(err) => {
            io::print_error(format!("failed to read '{}': {}", path.display(), err));
            return Vec::new();
        }
    };

    let valid_files: Vec<_> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|p| {
            if !p.is_file() {
                return false;
            }

            match p.extension().and_then(|e| e.to_str()) {
                Some(ext) => {
                    let ext = ext.to_ascii_lowercase();
                    ext == "csv" || ext == "xlsx"
                }
                None => false,
            }
        })
        .collect();

    if valid_files.is_empty() {
        io::print_error(format!(
            "'{}' does not contain any valid .csv or .xlsx files.",
            path.display()
        ));
    } else {
        io::print_info(format!(
            "{} valid file(s) found in '{}'.",
            valid_files.len(),
            path.display()
        ));
    }

    valid_files
}
