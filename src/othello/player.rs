// player.rs

use std::io;

use crate::util::{procs::*, values::*};

#[derive(Copy, Clone, Debug, Default)]
pub struct Player {
    pub color: i8,
    pub human: bool,
    pub passing: bool,
}

pub trait Bot {
    fn make_move(&self) -> isize;
}

impl Player {
    pub fn new() -> Player {
        let player: Player = Default::default();

        player
    }

    pub fn init(clr: i8, is_human: bool) -> Player {
        Player {
            color: clr,
            human: is_human,
            passing: false,
        }
    }

    pub fn get_input(&self, cells: Vec<usize>, human: bool) -> usize {
        let mut return_move = usize::max_value();
        let row: usize;
        let col: usize;

        let mut input = String::new();

        if !cells.is_empty() {
            println!("player has valid moves available");
        }

        print!("enter a move (color, column, row): ");
        io::stdin()
            .read_line(&mut input)
            .expect("unable to read user input");

        let chars: Vec<&str> = input.split_whitespace().collect();

        if chars[0] == "B" || chars[0] == "b" && !cells.is_empty() && input.len() > 1 {
            row = match ROWS.get(chars[2]) {
                Some(x) => *x,
                None => 256usize, // 256 if a bad read
            };

            col = match COLUMNS.get(chars[1]) {
                Some(x) => *x,
                None => 256usize, // 256 if a bad read
            };

            return_move = (row * 8) + col;
            if !cells.contains(&return_move) {
                if human {
                    println!("since a human is playing, please re-enter move");
                    self.get_input(cells, human);
                } else if !human {
                    eprintln!("invalid move entered");
                    std::process::exit(1);
                }
            } else {
                eprintln!("invalid move entered");
                std::process::exit(1);
            }

            println!("player {} made move ", get_color(self.color));
        }

        return_move
    }

    pub fn get_pass_input(&mut self, opponent: Player) {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("unable to read user input");

        match input.trim() {
            "B" | "b" => self.handle_skip_black(opponent),
            "W" | "w" => self.handle_skip_white(opponent),
            _ => {
                println!("invalid option found; please re-enter.");
                self.get_pass_input(opponent);
            }
        }
    }

    fn handle_skip_black(&mut self, mut opponent: Player) {
        if self.color != BLACK && self.human {
            println!("player has no valid moves and must pass; please re-enter:");
            self.get_pass_input(opponent);
        } else {
            match self.passing {
                true => opponent.passing = true,
                false => self.passing = true,
            }
        }
    }

    fn handle_skip_white(&mut self, mut opponent: Player) {
        if self.color != WHITE && self.human {
            println!("player has no valid moves and must pass; please re-enter:");
            self.get_pass_input(opponent);
        } else {
            match self.passing {
                true => opponent.passing = true,
                false => self.passing = true,
            }
        }
    }
}
