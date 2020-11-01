// bot.rs
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
