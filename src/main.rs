// main.rs

use othello_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut opts = pico_args::Arguments::from_env();
    let opts = othello_rs::Opts {
        help: opts.contains(["-h", "--help"]),
        version: opts.contains(["-v", "--version"]),
        human: opts.opt_value_from_str("--human")?.unwrap_or(true),
        debug: opts.contains(["-d", "--debug"]),
    };

    let game = othello::Board::new(); // this'll get initialized properly in the run function.

    othello_rs::run(game, opts)
    // here contains either an Ok(()) or an Error
}
