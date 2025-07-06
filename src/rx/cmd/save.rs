use clap::Args;
use polars::prelude::*;

use crate::rx::io;

#[derive(Args, Debug)]
pub struct SaveCmd {
    #[arg(value_name = "PATTERN")]
    pub pattern: String,

    #[arg(short = 'i', long)]
    pub ignore_case: bool,
}

pub fn run(cmd: SaveCmd, _dataset: &mut DataFrame) {
    io::print_info(format!(
        "filtering with pattern: '{}' (ignore_case: {})",
        cmd.pattern, cmd.ignore_case
    ));

    // implement logic to save the current dataset
}