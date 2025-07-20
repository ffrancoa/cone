use std::fs;
use std::error::Error;
use std::path::{Path, PathBuf};

use clap::{ArgGroup, Args};
use polars::prelude::*;

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

/// Executes the `load` command by validating and importing a file or directory.
pub fn run(cmd: LoadCmd, datasets: &mut Datasets) -> Result<bool, clap::Error> {
    let mut loaded_files = Vec::new();

    // validate and collect valid files from `-f`
    for path in &cmd.files {
        if let Some((name, file_path)) = validate_file_path(path, datasets) {
            match read_csv(&file_path) {
                Ok(df) => {
                    datasets.insert(name.clone(), df);
                    loaded_files.push((name, file_path));
                }
                Err(_) => {
                    io::print_error(format!(
                        "failed to load file '{}'", file_path.display(),
                    ));
                }
            }
        }
    }

    // validate and collect valid files from `-d`
    if let Some(dir_path) = &cmd.dir {
        let files_paths = validate_dir_path(dir_path);
        if !files_paths.is_empty() {
        }
        // TODO: "load files from directory"
    }

    if !loaded_files.is_empty() {
        for (name, path) in loaded_files {
            io::print_info(format!(
                "{} ← '{}' successfully loaded", name, path.display()
            ));
        }
    } else {
        io::print_error("no valid files were loaded");
    }

    Ok(true)
}

/*
fn ask_dataset_name(path: &Path, datasets: &Datasets) -> String {
    let default_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap()
        .to_string();

    loop {
        let Ok(input) = io::input_prompt("dataset name") else {
            io::print_error("failed to read input");
            io::print_error("please, try again");
            continue;
        };

        let name = input.trim();
        let final_name = if name.is_empty() {
            default_name.clone()
        } else {
            name.to_string()
        };

        if datasets.contains_key(&final_name) {
            io::print_error(format!("name '{final_name}' already exists"));
            io::print_error("please, choose a different one");
            continue;
        }

        return final_name
    }
}*/

/// Validates the path to a single file and prints errors if it is invalid.
fn validate_file_path(path: &Path, datasets: &Datasets) -> Option<(String, PathBuf)> {
    if !path.exists() {
        io::print_error(format!("'file {}' does not exist", path.display()));
        return None;
    }

    if !path.is_file() {
        io::print_error(format!("'{}' is not a file", path.display()));
        return None;
    }

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase);

    match ext.as_deref() {
        Some("csv") | Some("xlsx") => {
            // deduce name
            let name = match path.file_stem().and_then(|s| s.to_str()) {
                Some(s) => s.to_ascii_uppercase(),
                None => {
                    io::print_error(format!(
                        "could not extract a valid name from '{}'",
                        path.display()
                    ));
                    return None;
                }
            };

            if datasets.contains_key(&name) {
                io::print_error(format!(
                    "\"{}\" (from '{}') already exists in memory",
                    name,
                    path.display()
                ));
                return None;
            }

            Some((name, path.to_path_buf()))
        }
        _ => {
            io::print_error(format!(
                "file '{}' has an unsupported file extension",
                path.display()
            ));
            None
        }
    }
}

/// Validates a directory path and checks for CSV or XLSX files.
fn validate_dir_path(path: &PathBuf) -> Vec<PathBuf> {
    if !path.exists() {
        io::print_error(format!("directory '{}' does not exist", path.display()));
        return Vec::new();
    }

    if !path.is_dir() {
        io::print_error(format!("'{}' is not a directory", path.display()));
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
            "'{}' does not contain any valid .csv or .xlsx files",
            path.display()
        ));
    } else {
        io::print_info(format!(
            "{} valid file(s) found in '{}'",
            valid_files.len(),
            path.display()
        ));
    }

    valid_files
}

fn read_csv(file_path: &Path) -> Result<DataFrame, Box<dyn Error>> {
    let mut lazy_frame = LazyCsvReader::new(file_path)
        .with_infer_schema_length(Some(0))
        .finish()?;
    
    let schema = lazy_frame.collect_schema()?;

    let raw_df = lazy_frame
        .select(schema.iter_names().map(|name| {
            let sname = name.as_str();
            col(sname).cast(DataType::Float64).alias(sname)
            })
            .collect::<Vec<_>>()
        )
        .collect()?;

    Ok(raw_df)
}