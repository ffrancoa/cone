use clap::Args;

use crate::rx::io;
use crate::rx::Datasets;

#[derive(Args, Debug)]
pub struct PreviewCmd {
    /// Number of rows to be shown per dataset.
    #[arg(
        short, long, value_name = "ROWS",
        default_value_t = 6, value_parser = validate_nonzero,
        allow_hyphen_values = true
    )]
    rows: isize,
}

/// Executes the `preview` command by printing a partial view of
/// each dataset currently in memory.
pub fn run(cmd: PreviewCmd, datasets: &mut Datasets) -> Result<bool, clap::Error> {
    if datasets.is_empty() {
        io::print_error("no datasets have been loaded");
        return Ok(true);
    }

    for (df_name, df) in datasets.iter() {
        let n = cmd.rows;
        io::print_table(df, df_name, n);
    }

    Ok(true)
}

/// Ensure that the provided value is a non-zero integer.
fn validate_nonzero(s: &str) -> Result<isize, String> {
    let val: isize = s
        .parse()
        .map_err(|_| "argument must be an integer number")?;
    if val == 0 {
        Err("argument must be a non-zero integer number".into())
    } else {
        Ok(val)
    }
}