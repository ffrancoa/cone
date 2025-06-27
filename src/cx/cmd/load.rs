use std::path::PathBuf;

use clap::Args;
use polars::prelude::*;

use crate::cx::io;


/// Arguments for the `load` subcommand.
#[derive(Args, Debug)]
pub struct LoadCmd {
    /// Path of file to load.
    #[arg(
        short, long,
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