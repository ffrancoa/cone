use clap::Args;

//use crate::rx::io;
use crate::rx::Datasets;

#[derive(Args, Debug)]
pub struct SaveCmd {
    #[arg(value_name = "PATTERN")]
    pub pattern: String,

    #[arg(short = 'i', long)]
    pub ignore_case: bool,
}

pub fn run(_cmd: SaveCmd, _datasets: &mut Datasets) {
    // implement logic to save the current dataset
    todo!();
}