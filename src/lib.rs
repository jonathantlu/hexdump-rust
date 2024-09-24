use std::error::Error;
use std::fs;

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args.file_path)?;

    println!("{contents}");

    Ok(())
}

pub struct Args {
    len: Option<usize>,
    file_path: String,
}

impl Args {
    fn usage_message(program_name: &str) -> String {
        format!("Usage: {program_name} [-n LEN] FILE")
    }

    pub fn build(args: &[String]) -> Result<Args, String> {
        let name = &args[0];
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
                        len = match args[i].parse() {
                            Ok(len) => Some(len),
                            _ => {
                                return Err(format!("Invalid length: {}\n{}",
                                                   args[i],
                                                   Self::usage_message(name)));
                            }
                        };
                    } else {
                        return Err(Self::usage_message(name));
                    }
                }
                _ => {
                    if file_path.is_none() {
                        file_path = Some(args[i].clone());
                    }
                    // only one file path should be specified
                    // if there are more than one, only the first is considered
                }
            }
            i += 1
        }

        // make sure there was a filename in the input
        let file_path = match file_path {
            Some(file) => file,
            _ => {
                return Err(Self::usage_message(name));
            }
        };

        Ok(Args { len, file_path })
    }
}
