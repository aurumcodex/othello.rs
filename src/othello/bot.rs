// bot.rs
// use crate::othello::{player::*, Board};
// use crate::othello::{algorithms::*, player::*, Board};
#![allow(dead_code)] // will remove this at some point.
use crate::othello::{player::Player, Board};

enum MoveType {
    Auto, // automatically decide which move type would be best to use at current state
    RNG,
    AlphaBeta(bool), // fail-soft enabled or disabled
    Negamax,
    // MTDf, // may or may not be added in the future
}

pub trait Bot {
    fn make_move(&self, _game: Board, _turn_count: usize, _debug: bool) -> usize;
}

#[cfg(feature = "color")]
impl Bot for Player {
    fn make_move(&self, _game: Board, _turn_count: usize, _debug: bool) -> usize {
        // let mut rng = rand::
        let mut _best_move = usize::MAX;
        let _depth = 0;
        let _maxing = true;
        let _alpha = f64::MIN;
        let _beta = f64::MAX;
        let _color = self.color;

        let mut _move_type = MoveType::Auto;

        usize::MAX
    }
}

#[cfg(feature = "ascii")]
impl Bot for Player {
    fn make_move(&self, _game: Board, _turn_count: usize, _debug: bool) -> usize {
        // let mut rng = rand::
        let mut _best_move = usize::MAX;
        let _depth = 0;
        let _maxing = true;
        let _alpha = f64::MIN;
        let _beta = f64::MAX;
        let _color = self.color;

        let mut _move_type = MoveType::Auto;

        9999
    }
}

// helper functions

fn generate_alpha_beta_move(_game: Board, _alpha: f64, _beta: f64) -> usize {
    999
}

fn generate_negamax_move(_game: Board, _alpha: f64, _beta: f64) -> usize {
    999
}

// fn generate_mtdf_move(game: Board, alpha: f64, beta: f64) {}
