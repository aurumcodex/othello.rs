// lib.rs

pub mod othello;
pub mod util;

use othello::*;
use util::*;

#[derive(Debug)]
pub struct Opts {
    pub help: bool,
    pub version: bool,
    pub human: bool,
    pub debug: bool,
}

pub fn run(/*mut*/ game: Board, options: Opts) -> Result<(), Box<dyn std::error::Error>> {
    println!("Options are: {:?}", options);
    println!("Game Board: {:?}", game);

    if options.help {
        print_help();
    } else if options.version {
        print_verison();
    } /* else {
          play_othello(game);
      }
      */

    Ok(())
}

fn print_help() {
    println!("TODO: add a wonderful help message");
    std::process::exit(0);
}

fn print_verison() {
    println!("othello_rs - version: {}", values::VERSION);
    std::process::exit(0);
}
