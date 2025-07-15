use std::collections::HashMap;
use polars::prelude::DataFrame;

/// Map of named datasets stored in memory.
pub type Datasets = HashMap<String, DataFrame>;

pub mod cmd;
pub mod io;
pub mod repl;