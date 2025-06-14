use std::fmt::Display;

use crossterm::style::Stylize;

/// Prints a succesfull-operation message to stdout.
/// Styled with green text.
pub fn print_info(msg: impl Display) {
    println!("{} {}", "[info]:".bold().green(), msg.to_string().green())
}

/// Prints a warning message to stderr.
/// Styled with yellow text.
pub fn print_warn(msg: impl Display) {
    eprintln!("{} {}", "[warning]:".bold().yellow(), msg.to_string().yellow())
}

/// Prints an error message to stderr.
/// Styled with red text.
pub fn print_error(msg: impl Display) {
    eprintln!("{} {}", "[error]:".bold().red(), msg.to_string().red())
}