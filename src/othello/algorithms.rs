// algorithms.rs

use rand::prelude::*;

use crate::othello::moves::*;
use crate::util::values::BOARD_SIZE;

pub trait Algorithm {
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

    fn rng_move(moveset: Vec<Move>, debug: bool) -> usize {
        let cells = get_cells(moveset);
        let mut mv = rand::thread_rng().gen_range(0, BOARD_SIZE);

        if debug {
            println!("cell list: {:?}", &cells);
        }

        while !cells.contains(&mv) {
            mv = rand::thread_rng().gen_range(0, BOARD_SIZE);
        }
        mv
    }
}
