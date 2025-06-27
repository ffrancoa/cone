use std::error::Error;
use std::path::PathBuf;

use clap::Args;
use polars::prelude::*;

use crate::cx::io;

const REQUIRED_FILE_EXT: &str = "csv";

/// Arguments for the `load` subcommand.
#[derive(Args, Debug)]
pub struct LoadCmd {
    /// Path of file to load.
    #[arg(
        short, long,
        value_parser = file_parser,
        value_name = "FILE",
    )]
    file: Option<PathBuf>,

    /// Path of directory to load.
    #[arg(
        short, long,
        value_name = "DIR",
    )]
    dir: Option<PathBuf>,
}

pub fn run(cmd: LoadCmd, _dataset: &mut DataFrame) {
    if let Some(file) = cmd.file {
        io::print_info(format!("loading file: {:?}", file));
        // TODO: Load into `dataset` using Polars
    } else if let Some(dir) = cmd.dir {
        io::print_info(format!("loading directory: {:?}", dir));
        // TODO: Load multiple files if needed
    } else {
        io::print_error("no file or directory provided");
    }
}

/// A `clap`-compatible parser that always expects a `.csv` file.
fn file_parser(s: &str) -> Result<PathBuf, Box<dyn Error + Send + Sync + 'static>> {
    let path = PathBuf::from(s);

    // 1. check existence
    if !path.exists() { return Err("file does not exist".into()) }

    // 2. check extension (case‐insensitive)
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) if ext.eq_ignore_ascii_case(REQUIRED_FILE_EXT) => Ok(path),
        _ => Err("provided file does not have a valid extension".into()),
    }
}