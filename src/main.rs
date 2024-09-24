use std::env;

// Print the usage message and quit the process
fn print_usage_and_exit(program_name: &str) {
    eprintln!("Usage: {program_name} [-n LEN] FILE");
    std::process::exit(1);
}

fn main() {
    // Read in command line arguments
    let args: Vec<String> = env::args().collect();

    let program_name = &args[0];
    let mut len: Option<usize> = None;
    let mut file_path: Option<String> = None;

    // loop through command line arguments to check for optional flags
    let mut i = 1;
    while i < args.len() {
        // match the argument with either the "-n" option or FILE
        match args[i].as_str() {
            "-n" => {
                i += 1;
                // check if length exists, and if it is valid
                if i < args.len() {
                    len = Some(args[i].parse().unwrap_or_else(|error| {
                        eprintln!("Invalid length: {}", args[i]);
                        print_usage_and_exit(program_name);
                        0  // will not be reached
                    }));
                } else {
                    print_usage_and_exit(program_name);
                }
            }
            _ => {
                if file_path.is_none() {
                    file_path = Some(args[i].clone());
                } else {
                    // only one file path should be specified
                    print_usage_and_exit(program_name);
                }
            }
        }
        i += 1
    }

    // make sure there was a filename in the input
    if file_path.is_none() {
        print_usage_and_exit(program_name);
    }

}
