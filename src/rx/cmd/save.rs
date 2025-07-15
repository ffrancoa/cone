use std::collections::HashMap;

use clap::Args;
use polars::prelude::*;

//use crate::rx::io;

#[derive(Args, Debug)]
pub struct SaveCmd {
    #[arg(value_name = "PATTERN")]
    pub pattern: String,

    #[arg(short = 'i', long)]
    pub ignore_case: bool,
}

pub fn run(_cmd: SaveCmd, _datasets: &mut HashMap<String, DataFrame>) {
    // implement logic to save the current dataset
    todo!();
}