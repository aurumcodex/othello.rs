// lib.rs

// This Othello implementation uses Rust's `i8` data type for its 'Color' representation

pub mod othello;
pub mod util;

use std::error::Error;

use othello::{bot::Bot, Board};
use util::*;

#[derive(Debug)]
pub struct Opts {
    pub help: bool,
    pub version: bool,
    pub human: bool,
    pub debug: bool,
    pub algorithm: String,
}

pub fn run(/*mut*/ game: Board, options: Opts) -> Result<(), Box<dyn Error>> {
    println!("Options are: {:?}", options);
    println!("Game Board: {:?}", game);

    let mv = game.player().make_move(game, 9, true);

    if options.help {
        print_help()
    } else if options.version {
        print_verison()
    } else {
        println!("{}", mv);
        play_othello(game, options.human, options.debug)
    }
}

fn play_othello(
    /*mut*/ _game: Board,
    _human: bool,
    _debug: bool,
) -> Result<(), Box<dyn Error>> {
    // TODO: implement main game loop here (rather important)
    Ok(())
}

fn print_help() -> Result<(), Box<dyn Error>> {
    println!("{}", &HELP);
    std::process::exit(0);
}

fn print_verison() -> Result<(), Box<dyn Error>> {
    println!("othello_rs - version: {}", values::VERSION);
    std::process::exit(0);
}
