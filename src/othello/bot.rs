// bot.rs
// use crate::othello::{player::*, Board};
// use crate::othello::{algorithms::*, player::*, Board};
#![allow(dead_code)] // will remove this at some point.
use crate::othello::Board;

enum MoveType {
    Auto, // automatically decide which move type would be best to use at current state
    RNG,
    AlphaBeta(bool), // fail-soft enabled or disabled
    Negamax,
    // MTDf, // may or may not be added in the future
}

// imlpmenting the move making for the bot here instead of in player.rs
// mainly because it's only used for a single method.
// "implemented" in player.rs without any function declarations
pub trait Bot {
    fn make_move(&self, _game: Board, _turn_count: usize, _debug: bool) -> usize {
        42
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
