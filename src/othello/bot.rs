// bot.rs
#![allow(dead_code)]
// use crate::othello::{algorithms::*, player::*, Board};
use crate::othello::player::*;

enum MoveType {
    RNG,
    AlphaBeta,
    Negamax,
    MTDf,
    SSS,
}

impl Bot for Player {
    fn make_move(&self) -> isize {
        0
    }
}
