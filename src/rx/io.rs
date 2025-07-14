use std::fmt::Display;

use clap::{crate_description, crate_name};
use crossterm::{
    style::{StyledContent, Stylize},
    terminal,
};
use tabled::{
    Table,
    settings::{Alignment, Remove, Style, Width},
    settings::object::Rows,
};

/// Maximum width for app rendering.
const APP_WIDTH_LIMIT: usize = 80;

/// Determines the width to use for the header, limited by APP_WIDTH_LIMIT.
fn app_width() -> usize {
    terminal::size()
        .map(|(width, _)| usize::min(APP_WIDTH_LIMIT, width as usize))
        .unwrap_or(APP_WIDTH_LIMIT)
}

/// Applies styling to parts of the input string enclosed
/// in single quotes, including the quotes.
fn stylize_single_quoted(input: &str) -> Vec<StyledContent<String>> {
    let mut parts = Vec::new();
    let mut buffer = String::new();
    let mut in_quotes = false;

    for c in input.chars() {
        match c {
            '\'' => {
                buffer.push('\'');
                if in_quotes {
                    parts.push(buffer.clone().dark_yellow());
                    buffer.clear();
                    in_quotes = false;
                } else {
                    if !buffer.is_empty() {
                        parts.push(buffer.clone().reset());
                        buffer.clear();
                    }
                    in_quotes = true;
                }
            }
            _ => buffer.push(c),
        }
    }

    if !buffer.is_empty() {
        parts.push(buffer.reset());
    }

    parts
}

/// Prints a styled message with highlighted quoted text.
/// Routes output to either stdout or stderr.
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
/// styled with green text. Text within single quotes is highlighted.
pub fn print_info(msg: impl Display) {
    let label = "info:".to_string().bold().green();
    print_styled_msg(label, msg, false);
}

/// Prints a warning message to stderr,
/// styled with yellow text. Text within single quotes is highlighted.
pub fn print_warn(msg: impl Display) {
    let label = "warning:".to_string().bold().yellow();
    print_styled_msg(label, msg, true);
}

/// Prints an error message to stderr,
/// styled with red text. Text within single quotes is highlighted.
pub fn print_error(msg: impl Display) {
    let label = "error:".to_string().bold().red();
    print_styled_msg(label, msg, true);
}
