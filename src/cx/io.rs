use std::fmt::Display;

use crossterm::style::Stylize;

pub fn print_info(msg: impl Display) {
    println!("{} {}", "[Info]:".bold().green(), msg)
}

pub fn print_error(msg: impl Display) {
    eprintln!("{} {}", "[Error]:".bold().red(), msg)
}