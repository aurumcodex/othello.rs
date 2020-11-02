// (othello) mod.rs

pub mod algorithms;
pub mod bot;
pub mod moves;
pub mod player;
pub mod types;

use crate::util::{procs::*, values::*};

// use algorithms::*;
use moves::*;
use player::*;

#[derive(Copy, Clone, Debug)]
pub struct Board {
    player: Player,
    bot: Player,
    board: [i8; BOARD_SIZE],
    game_over: bool,
}

pub trait Algorithm {
    fn alpha_beta(
        board: &mut Self,
        alpha: &mut f64,
        beta: &mut f64,
        color: i8,
        depth: isize,
        maxing: bool,
        debug: bool,
    ) -> isize;

    fn negamax(
        board: &mut Self,
        alpha: &mut f64,
        beta: &mut f64,
        color: i8,
        depth: isize,
        debug: bool,
    ) -> isize;

    fn rng_move(moveset: Vec<Move>, debug: bool) -> isize;
}

impl Default for Board {
    fn default() -> Self {
        Board {
            player: Player::new(),
            bot: Player::new(),
            board: [0; 64],
            game_over: false,
        }
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            player: Player::new(),
            bot: Player::new(),
            board: [NONE; BOARD_SIZE],
            game_over: false,
        }
    } // generate new board

    pub fn setup(&mut self, color: i8, debug: bool) {
        self.player = Player::init(color, true);
        self.bot = Player::init(color, false);

        if debug {
            println!(
                "player set up as {}; bot set up as {}",
                get_color(self.player.color),
                get_color(self.bot.color)
            );
        }
    } // setup board via modifying variables

    pub fn apply(&mut self, _color: i8, _cell: usize, _debug: bool) {}

    pub fn flip_discs(&mut self, color: i8, cell: usize, dir: isize, debug: bool) {
        let mut temp = cell as isize;

        while temp >= 0 && cell < BOARD_SIZE {
            temp += dir;

            if debug {
                println!("cell is: {}", temp);
            }

            if temp < 64 {
                if self.board[temp as usize] == color {
                    break;
                } else {
                    self.board[temp as usize] = color;
                }
            }
        }
    }

    // this is for printing out the board *with* all the available moves on it.
    pub fn display(&self, _moveset: Vec<Move>) {}

    // this one is a printing function that just shows the board, without any moves on it.
    pub fn show(&self) {}

    pub fn is_game_over(&self) -> bool {
        self.player.passing == self.bot.passing
    } // check if the game is over
}

impl Movelist for Board {
    fn get_legal_move(&self, mut index: isize, dir: isize, color: i8) -> Move {
        let mut flips = 0;
        let mut wall = false;
        let mut result: Move = Default::default();

        while index >= 0 && index < BOARD_SIZE as isize && !wall {
            wall = check_wall(index as usize, dir);

            index += dir;

            if index >= 0 && index < BOARD_SIZE as isize {
                if self.board[index as usize] != -color {
                    break;
                } else {
                    flips += 1;
                }
            } else {
                flips = 0;
                break;
            }
        }

        if index >= 0
            && index < BOARD_SIZE as isize
            && self.board[index as usize] == NONE
            && flips != 0
        {
            result.cell = index as usize;
            result.num_flips = flips;
            result.direction = dir;
        }

        result
    } // gets a legal move from the board

    fn generate_moves(&self, color: i8) -> Vec<Move> {
        let mut movelist: Vec<Move> = Vec::new();

        for (i, val) in self.board.iter().enumerate() {
            if val == &color {
                for dir in &DIRECTIONS {
                    let mv = self.get_legal_move(i as isize, *dir, color);

                    if mv.num_flips != 0 && !mv.is_border() {
                        movelist.push(mv);
                    }
                }
            }
        }
        movelist
    } // generates a vector of legal moves on the board
}
