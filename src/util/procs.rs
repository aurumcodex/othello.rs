// procs.rs

use crate::util::values::{BLACK, NONE, WHITE};

pub fn print_char(i: usize, s: &str) {
    if i % 8 == 7 {
        println!(" {}", s);
    } else {
        print!(" {}", s);
    }
}

pub fn get_color(color: i8) -> String {
    let result = match color {
        BLACK => String::from("B"),
        WHITE => String::from("W"),
        NONE => String::from("-"),
        _ => String::from("?"),
    };

    result
}

pub fn get_row(x: i8) -> usize {
    0
}

pub fn get_col(x: i8) -> usize {
    0
}
