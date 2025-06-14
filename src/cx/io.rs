use std::fmt::Display;

use crossterm::style::Stylize;

pub fn print_info(msg: impl Display) {
    println!("{} {}", "[info]:".bold().green(), msg)
}

pub fn print_warn(msg: impl Display) {
    println!("{} {}", "[warning]:".bold().yellow(), msg)
}

pub fn print_error(msg: impl Display) {
    eprintln!("{} {}", "[error]:".bold().red(), msg)
}