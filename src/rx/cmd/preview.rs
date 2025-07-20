use clap::Args;

use crate::rx::io;
use crate::rx::Datasets;

#[derive(Args, Debug)]
pub struct PreviewCmd {
    /// Number of rows to be shown.
    #[arg(
        short, long, value_name = "ROWS",
        default_value_t = 6,  allow_hyphen_values = true
    )]
    rows: isize,
}

pub fn run(cmd: PreviewCmd, datasets: &mut Datasets) {
    if datasets.is_empty() {
        io::print_error("no datasets have been loaded");
        return
    }

    for (df_name, df) in datasets.iter() {
        io::print_table(df, df_name, cmd.rows);
    }
}