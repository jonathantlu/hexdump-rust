use std::env;
use std::process;
use std::io::ErrorKind;

use hexdump::Args;

fn main() {
    // Read in command line arguments
    let args: Vec<String> = env::args().collect();

    // parse the command line arguments
    let args = Args::parse(&args).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    // run application code on args
    if let Err(e) = hexdump::run(&args) {
        match e.kind() {
            ErrorKind::NotFound | ErrorKind::PermissionDenied => {
                eprintln!("{}: {}: {e}", args.program_name(), args.file_path());
            }
            _ => {
                eprintln!("{}: {e}", args.program_name());
            }
        }
        process::exit(1);
    }
}
