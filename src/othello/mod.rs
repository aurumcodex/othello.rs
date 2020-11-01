// (othello) mod.rs
pub mod algorithms;
pub mod bot;
pub mod moves;
pub mod player;
pub mod types;

use crate::util::{procs::*, values::*};

use algorithms::*;
use moves::*;
use player::*;

#[derive(Debug)]
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

impl Board {
    pub fn new() -> Board {
        Board {
            player: Player::new(),
            bot: Player::new(),
            board: [NONE; BOARD_SIZE],
            game_over: false,
        }
    }

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
    }

    pub fn apply(&mut self, color: i8, cell: usize, debug: bool) {}

    pub fn flip_discs(&mut self, color: i8, cell: usize, dir: isize, debug: bool) {}

    pub fn display(&self, moveset: Vec<Move>) {}

    // this one is a printing function that just shows the board, without any moves on it.
    pub fn show(&self) {}

    pub fn is_game_over(&self) -> bool {
        self.player.passing == self.bot.passing
    }
}

impl Movelist for Board {
    fn get_legal_move(&self, mut index: isize, dir: isize, color: i8) -> Move {
        Move {
            cell: 0,
            num_flips: 0,
            direction: 9,
        }
    }

    fn generate_moves(&self, color: i8) -> Vec<Move> {
        let movelist: Vec<Move> = Vec::new();

        movelist
    }
}
