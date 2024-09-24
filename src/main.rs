use std::env;
use std::process;

use hexdump::Args;

fn main() {
    // Read in command line arguments
    let args: Vec<String> = env::args().collect();

    // parse the command line arguments
    let args = Args::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    // run application code on args
    if let Err(e) = hexdump::run(args) {
        eprintln!("Application error {e}");
        process::exit(1);
    }
}
