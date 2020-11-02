// algorithms.rs

use crate::othello::{
    moves::{get_cells, Move},
    Algorithm, Board,
};

impl Algorithm for Board {
    fn alpha_beta(
        _board: &mut Self,
        _alpha: &mut f64,
        _beta: &mut f64,
        _color: i8,
        _depth: isize,
        _maxing: bool,
        _debug: bool,
    ) -> isize {
        0
    }

    fn negamax(
        _board: &mut Self,
        _alpha: &mut f64,
        _beta: &mut f64,
        _color: i8,
        _depth: isize,
        _debug: bool,
    ) -> isize {
        0
    }

    fn rng_move(moveset: Vec<Move>, _debug: bool) -> isize {
        let /*mut*/ _cells = get_cells(moveset);
        0
    }
}
