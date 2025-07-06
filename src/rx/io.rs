use std::fmt::Display;

use clap::{crate_name, crate_description};
use crossterm::terminal;
use crossterm::style::Stylize;
use tabled::{
    Table,
    settings::{Alignment, Remove, Style, Width},
    settings::object::Rows, 
};

/// Maximum width for app rendering.
const APP_WIDTH_LIMIT: usize = 80;

/// Determine the width to use for the header, limited by APP_WIDTH_LIMIT.
fn app_width() -> usize {
    terminal::size()
        .map(|(width, _)| usize::min(APP_WIDTH_LIMIT, width as usize))
        .unwrap_or(APP_WIDTH_LIMIT)
}

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


/// Prints a successful-operation message to stdout,
/// styled with green text.
pub fn print_info(msg: impl Display) {
    println!("{} {}", "info:".bold().green(), msg)
}

/// Prints a warning message to stderr,
/// styled with yellow text.
pub fn print_warn(msg: impl Display) {
    eprintln!("{} {}", "warning:".bold().yellow(), msg)
}

/// Prints an error message to stderr,
/// styled with red text.
pub fn print_error(msg: impl Display) {
    eprintln!("{} {}", "error:".bold().red(), msg)
}