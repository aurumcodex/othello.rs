// player.rs

// use std::collections::HashMap;
use std::io;

// use crate::type_of;
use crate::util::{procs::*, values::*};

#[derive(Copy, Clone, Debug, Default)]
pub struct Player {
    pub color: i8,
    pub human: bool,
    pub passing: bool,
}

// impl Bot for Player {}

impl Player {
    pub fn new() -> Player {
        Player::default()
    }

    pub fn init(clr: i8, is_human: bool) -> Player {
        Player {
            color: clr,
            human: is_human,
            passing: false,
        }
    }

    pub fn get_input(&self, cells: Vec<usize>, human: bool) -> usize {
        let return_move: usize; // = usize::max_value();
        let row: usize;
        let col: usize;

        let mut input = String::new();

        if !cells.is_empty() {
            println!("player has valid moves available");
        }

        println!("enter a move (color, column, row): ");
        io::stdin()
            .read_line(&mut input)
            .expect("unable to read user input");

        let chars: Vec<&str> = input.split_whitespace().collect();
        println!("{:?}", chars);

        if chars[0] == "B" || chars[0] == "b" && !cells.is_empty() && input.len() > 1 {
            row = match ROWS.get(chars[2]) {
                Some(x) => *x,
                None => 256usize, // 256 if a bad read
            };

            col = match COLUMNS.get(chars[1]) {
                Some(x) => *x,
                None => 256usize, // 256 if a bad read
            };

            // let in_col = chars[1];
            // let in_row = chars[2];
            // println!(
            //     "has keys? row {} col {}",
            //     ROWS.contains_key(chars[2]),
            //     ROWS.contains_key(chars[1])
            // );
            // println!(
            //     "{}: {}, {}: {}",
            //     type_of(chars[2]),
            //     chars[2],
            //     type_of(chars[1]),
            //     chars[1]
            // );
            // row = *ROWS.get(in_row).unwrap();
            // // row = 256;
            // col = *COLUMNS.get(in_col).unwrap();
            println!("row {}, col {}", row + 1, col + 1);

            return_move = (row * 8) + col;
            println!("{}", return_move);
            if !cells.contains(&return_move) {
                if human {
                    println!("since a human is playing, please re-enter move");
                    self.get_input(cells, human);
                } else if !human {
                    eprintln!("invalid move entered");
                    std::process::exit(1);
                }
            }

            println!("player {} made move {}", get_color(self.color), return_move);
        } else {
            eprintln!("invalid move entered (ie)");
            std::process::exit(1);
        }

        return_move
    }
}

pub trait Passing {
    fn get_pass_input(&mut self);

    fn handle_skip_black(&mut self, opponent: Player);

    fn handle_skip_white(&mut self, opponent: Player);
}
