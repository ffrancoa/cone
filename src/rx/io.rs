use std::io::{self, Write};
use std::fmt::Display;

use clap::{crate_description, crate_name};
use crossterm::{
    style::{StyledContent, Stylize},
    terminal,
};
use polars::prelude::DataFrame;
use tabled::Table;
use tabled::{
    builder,
    settings::{Alignment, Border, Color, Remove, Style, Width},
    settings::object::Rows,
    settings::style::BorderColor,
};

/// Prompt symbol for the REPL interface.
pub const PROMPT: &str = "❯";

/// Maximum width for app rendering.
const APP_WIDTH_LIMIT: usize = 88;

/// Number of decimal places to show in tables.
const TABLE_FLOAT_PRECISION: usize = 2;

/// Prints a styled header banner above the REPL interface.
///
/// The header shows either the crate name or description,
/// depending on terminal width.
pub fn header(app_code: &str) {
    // define title based on available width
    let app_title = if app_width() < 60 {
        crate_name!().to_ascii_uppercase()
    } else {
        crate_description!().to_owned()
    };

    let title_msg = format!("{app_code} .::. {app_title}");

    let mut table = Table::new(vec![title_msg]);
    table.with(Style::extended());
    table.with(Alignment::center());
    table.with(Remove::row(Rows::first()));
    table.with(Width::increase(app_width()));

    println!("\n{}", table.to_string().bold().blue());
}


/// Reads user input with a custom prompt message.
///
/// Returns the input string or IO error if reading fails.
pub fn _input_prompt(msg: &str) -> Result<String, io::Error> {
    let in_prompt = format!("IN {PROMPT} ").bold().yellow();
    let message = format!("{}{} ", msg.yellow(), ":".bold().yellow());

    print!("{in_prompt} {message}");
    io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(input)
}

/// Prints a successful-operation message to stdout.
///
/// The message is styled with green text. Text within
/// single quotes is highlighted in dark yellow.
pub fn print_info(msg: impl Display) {
    let label = "info:".to_string().bold().green();
    print_styled_msg(label, msg, false);
}

/// Prints a warning message to stderr.
///
/// The message is styled with yellow text. Text within
/// single quotes is highlighted in dark yellow
pub fn print_warn(msg: impl Display) {
    let label = "warning:".to_string().bold().yellow();
    print_styled_msg(label, msg, true);
}

/// Prints an error message to stderr,
///
/// The message is styled with red text. Text within
/// single quotes is highlighted in dark yellow.
pub fn print_error(msg: impl Display) {
    let label = "error:".to_string().bold().red();
    print_styled_msg(label, msg, true);
}

/// Determines the width to use for the header, limited by APP_WIDTH_LIMIT.
fn app_width() -> usize {
    terminal::size()
        .map(|(width, _)| usize::min(APP_WIDTH_LIMIT, width as usize))
        .unwrap_or(APP_WIDTH_LIMIT)
}

/// Prints a dataframe as a formatted table with headers and units.
///
/// The table shows either first or last N rows based on the sign of `nrows`.
pub fn print_table(data: &DataFrame, name: &str, nrows: isize) {
    let count = nrows.unsigned_abs();
    let total_rows = data.height();

    if count > total_rows {
        print_error("cannot print more rows than dataset height");
        return
    }

    let sliced = if nrows > 0 {
            data.head(Some(count))
        } else {
            data.tail(Some(count))
        };

    // extract labels and units from column names
    let (labels, units): (Vec<String>, Vec<String>) = data
        .get_column_names()
        .iter()
        .map(|s| {
            if let Some((label, unit)) = s.rsplit_once('(') {
                (
                    label.trim().to_string(),
                    format!("({})", unit.trim().trim_end_matches(')')),
                )
            } else {
                // fallback if there's no unit
                (s.to_string(), String::new())
            }
        })
        .unzip();

    let mut rows: Vec<Vec<String>> = (0..sliced.height())
        .filter_map(|i| build_row(i, &sliced))
        .collect();

    rows.insert(0, labels);
    rows.insert(1, units);

    let mut builder = builder::Builder::default();
    for row in rows {
        builder.push_record(row);
    }

    // visual table tweaks
    let mut table = builder.build();
    table
        .with(Style::re_structured_text())
        .with(Alignment::right())
        .modify(Rows::one(0), Border::empty())
        .modify(Rows::one(0), Color::BOLD)
        .modify(Rows::one(0), BorderColor::filled(Color::FG_BRIGHT_BLACK))
        .modify(Rows::one(0), Alignment::center())
        .modify(Rows::one(1), Border::new().bottom('='))
        .modify(Rows::one(1), BorderColor::filled(Color::FG_BRIGHT_BLACK))
        .modify(Rows::one(1), Alignment::center())
        .modify(Rows::last(), BorderColor::filled(Color::FG_BRIGHT_BLACK))
        .with(Width::increase(app_width()));
        

    let plural = if count == 1 { "row" } else { "rows" };
    let prefix = if nrows > 0 { "first" } else { "last" };
    let subtitle = format!("({prefix} {count} {plural})");
    let title = format!("\n {PROMPT} {PROMPT} {name}");

    println!("{} {}", title.bold(), subtitle);
    println!("{table}");
}

/// Builds a single table row from DataFrame values.
fn build_row(i: usize, df: &DataFrame) -> Option<Vec<String>> {
    df.get_columns().iter().map(|col| {
        let val = col.get(i).inspect_err(|_| {
            print_error(format!("missing value at row {i}"));
        }).ok()?;

        let fval = val.try_extract::<f64>().inspect_err(|_| {
            print_error(format!("non-f64 value at row {i}"));
        }).ok()?;

        Some(format!("{fval:.TABLE_FLOAT_PRECISION$}"))
    }).collect()
}

/// Applies styling to parts of the input string enclosed in single quotes.
///
/// Returns a vector of styled content segments where text within quotes
/// is highlighted while maintaining the original quote characters.
fn stylize_single_quoted(input: &str) -> Vec<StyledContent<String>> {
    let mut parts = Vec::new();
    let mut buffer = String::new();
    let mut in_quotes = false;

    for c in input.chars() {
        match c {
            '\'' => {
                if in_quotes {
                    // add styled quoted content
                    parts.push(buffer.clone().dark_yellow());
                    buffer.clear();
                    in_quotes = false;
                } else {
                    // add previous non-quoted content
                    if !buffer.is_empty() {
                        parts.push(buffer.clone().reset());
                        buffer.clear();
                    }
                    in_quotes = true;
                }

                // always add the quote itself with default style
                parts.push("'".to_string().reset());
            }
            _ => buffer.push(c),
        }
    }

    // push any remaining non-quoted text
    if !buffer.is_empty() {
        parts.push(buffer.reset());
    }

    parts
}

/// Prints a styled message with highlighted quoted text.
fn print_styled_msg(
    styled_label: StyledContent<String>,
    msg: impl Display,
    use_stderr: bool,
) {
    if use_stderr {
        eprint!("{styled_label} ");
        for part in stylize_single_quoted(&msg.to_string()) {
            eprint!("{part}");
        }
        eprintln!();
    } else {
        print!("{styled_label} ");
        for part in stylize_single_quoted(&msg.to_string()) {
            print!("{part}");
        }
        println!();
    }
}