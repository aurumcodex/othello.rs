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
    match color {
        BLACK => String::from("B"),
        WHITE => String::from("W"),
        NONE => String::from("-"),
        _ => String::from("?"),
    }
}

pub fn get_row(x: usize) -> usize {
    (x / 8) + 1
}

pub fn get_col(x: usize) -> String {
    match x % 8 {
        0 => String::from("a"),
        1 => String::from("b"),
        2 => String::from("c"),
        3 => String::from("d"),
        4 => String::from("e"),
        5 => String::from("f"),
        6 => String::from("g"),
        7 => String::from("h"),
        _ => String::from("_"),
    }
}
