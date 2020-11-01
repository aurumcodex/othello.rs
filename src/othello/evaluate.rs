// evaluate.rs
use crate::othello::Board;

use crate::util::values::*;

pub struct Scores {
    black: isize,
    white: isize,
    score: isize,
}

pub fn calculate_scores_disc(board: Board) -> Scores {
    let mut black_count = 0;
    let mut white_count = 0;

    for cell in board.board {
        match cell {
            BLACK => black_count++,
            WHITE => white_count++,
            _ => {},
        }
    }

    let score = black_count - white_count;

    Scores {
        black: black_count,
        white: white_count,
        score: score
    }
}

pub fn calculate_scores_weight(board: Board) -> Scores {
    let mut black_count = 0;
    let mut white_count = 0;

    for i, cell in board.board.iter().enumerate() {
        match cell {
            BLACK => black_count += WEIGHTS[i],
            WHITE => white_count += WEIGHTS[i],
            _ => {},
        }
    }

    let score = black_count - white_count;
    
    Scores {
        black: black_count,
        white: white_count,
        score: score
    }
}

pub fn print_results(s: Scores) {
    if s.black > s.white {
        println!("player black wins.");
        println!("black pieces: {}", s.black);
        println!("white pieces: {}", s.white);
    } else if s.black < s.white {
        println!("player white wins.");
        println!("black pieces: {}", s.black);
        println!("white pieces: {}", s.white);
    } else {
        println!("a tie occurred.");
        println!("black pieces: {}", s.black);
        println!("white pieces: {}", s.white);
    }
}