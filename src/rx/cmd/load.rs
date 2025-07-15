use std::fs;
use std::path::{Path, PathBuf};

use clap::{ArgGroup, Args};

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
        if let Some(valid_path) = validate_file_path(path) {
            targets.files.push(valid_path);
        }
        // TODO: load into `dataset` using Polars
    }

    // validate and collect valid files from `-d`
    if let Some(path) = &cmd.dir {
        let files_paths = validate_dir_path(path);
        if !files_paths.is_empty() {
            targets.files_from_dir = files_paths;
        }
        // TODO: load multiple files from directory
    }

    Ok(true)
}

/// Validates the path to a single file and prints errors if it is invalid.
fn validate_file_path(path: &Path) -> Option<PathBuf> {
    if !path.exists() {
        io::print_error(format!("invalid value: '{}'", path.display()));
        io::print_error("file does not exist");
        return None;
    }

    if !path.is_file() {
        io::print_error(format!("invalid value: '{}'", path.display()));
        io::print_error("provided path does not correspond to a file");
        return None;
    }

    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase());

    // if the file exists, it must have a CSV or XLSX extension
    let valid = matches!(extension.as_deref(), Some("csv") | Some("xlsx"));

    if !valid {
        io::print_error(format!("invalid value: '{}'", path.display()));
        io::print_error("provided file does not have a valid extension");
        None
    } else {
        io::print_info(format!("loading file: {}", path.display()));
        Some(path.to_path_buf())
    }
}

/// Validates a directory path and checks for CSV or XLSX files.
fn validate_dir_path(path: &PathBuf) -> Vec<PathBuf> {
    if !path.exists() {
        io::print_error(format!("invalid value: '{}'", path.display()));
        io::print_error("directory does not exist");
        return Vec::new();
    }

    if !path.is_dir() {
        io::print_error(format!("invalid value: '{}'", path.display()));
        io::print_error("provided path is not a directory");
        return Vec::new();
    }

    // search for at least one valid file
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => {
            io::print_error(format!("cannot read directory: '{}'", path.display()));
            return Vec::new();
        }
    };

    let valid_files: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| matches!(e.to_ascii_lowercase().as_str(), "csv" | "xlsx"))
                    .unwrap_or(false)
        })
        .collect();

    if valid_files.is_empty() {
        io::print_error(format!("invalid value: '{}'", path.display()));
        io::print_error("directory does not contain any valid .csv or .xlsx files");
    } else {
        io::print_info(format!(
            "loading directory: '{}' ({} file(s) found)",
            path.display(),
            valid_files.len()
        ));
    }

    valid_files
}
