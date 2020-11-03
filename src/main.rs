// main.rs

use othello_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut opts = pico_args::Arguments::from_env();
    let opts = othello_rs::Opts {
        help: opts.contains(["-h", "--help"]),
        version: opts.contains(["-v", "--version"]),
        human: opts.opt_value_from_str("--human")?.unwrap_or(true),
        debug: opts.contains(["-d", "--debug"]),
        algorithm: opts
            .opt_value_from_str("--algorithm")?
            .unwrap_or_else(|| "auto".into()),
    };

    let game = othello::Board::new(); // this'll get initialized properly in the run function.

    othello_rs::run(game, opts)
    // function call above returns a result like the one the main does
} // end main
