use std::fmt::Display;

pub fn print_info(msg: impl Display) {
    let info_msg = format!("[info]: {}", msg);
    println!("{}", info_msg)
}

pub fn print_error(msg: impl Display) {
    let error_msg = format!("[error]: {}", msg);
    eprintln!("{}", error_msg)
}