use std::collections::HashMap;

use clap::Args;
use polars::prelude::*;

//use crate::rx::io;

#[derive(Args, Debug)]
pub struct PreviewCmd {
    // #[arg(short, long, default_value = "10")]
    // pub lines: usize,
}

pub fn run(_cmd: PreviewCmd, _datasets: &mut HashMap<String, DataFrame>) {
    /*
    if dataset.height() == 0 {
        io::print_warn("No data loaded.");
    } else {
        io::print_info("Previewing dataset...");
        println!("{:?}", dataset.head(Some(5))); // example
    }
    */
}