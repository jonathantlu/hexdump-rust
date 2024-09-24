use std::fs::File;
use std::io::{self, BufReader, Read};

const BYTES_PER_LINE: usize = 16;

// main application code
pub fn run(args: &Args) -> io::Result<()> {
    // attempt to open the file
    let file = File::open(args.file_path.as_str())?;
    
    // if a len has been specified, use it
    if let Some(n) = args.len {
        return read_convert_print(file.take(n));
    }
    read_convert_print(file)
}

// function that accepts any type that implements Read
// because file and file.take have different types, but both impl Read
fn read_convert_print(file: impl Read) -> io::Result<()> {
    // use BufReader as we will need to read many times
    let mut reader = BufReader::new(file);
    let mut buf: [u8; BYTES_PER_LINE] = [0; BYTES_PER_LINE];
    
    let mut offset = 0usize;
    // read BYTES_PER_LINE bytes from the file
    let mut n = reader.read(&mut buf)?;
    // need to keep reading until 0 is returned, meaning EOF is reached
    while n != 0 {
        // for each set of bytes read, convert to hex and print
        println!("{offset:08x} {}", convert_to_hexdump_line(&buf, n));
        offset += n;
        n = reader.read(&mut buf)?;
    }

    // print final offset
    println!("{offset:08x}");

    Ok(())
}

// takes a buf array of size n, converts it to hex string
fn convert_to_hexdump_line(buf: &[u8], n: usize) -> String {
    let mut line = Vec::new();

    // each seperated number is 4 digits long, so 2 bytes each
    for i in (0..BYTES_PER_LINE).step_by(2) {
        if i + 1 < n {
            // builds in little endian
            // TODO: add a flag to force big endian
            let in_hex = (buf[i + 1] as u16) << 8 | buf[i] as u16;
            line.push(format!("{in_hex:04x}"));
        } else if i < n {
            line.push(format!("{:04x}", buf[i]));
        } else {
            // the final line has extra spaces to match the other lines
            line.push(String::from("    "));
        }
    }

    line.join(" ")
}

pub struct Args {
    program_name: String,
    len: Option<u64>,
    file_path: String,
}

impl Args {
    fn usage_message(program_name: &str) -> String {
        format!("Usage: {program_name} [-n LEN] FILE")
    }

    pub fn parse(args: &[String]) -> Result<Args, String> {
        let name = &args[0];
        let mut len: Option<u64> = None;
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

        Ok(Args { program_name: String::from(name), len, file_path })
    }

    // getter for file name
    pub fn file_path(&self) -> &str {
        self.file_path.as_str()
    }

    // getter for program_name
    pub fn program_name(&self) -> &str {
        self.program_name.as_str()
    }
}

// testing code
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_args() {
        let args = vec![String::from("./hexdump")];
        match Args::parse(&args) {
            Err(e) => {
                assert_eq!(e.as_str(), "Usage: ./hexdump [-n LEN] FILE");
            }
            _ => {
                panic!();
            }
        }
    }

    // arg parse testing
    #[test]
    fn working_args() {
        let args = vec![String::from("./hexdump"), String::from("haiku.txt")];
        let args = Args::parse(&args).unwrap();
        assert_eq!(args.len, None);
        assert_eq!(args.file_path.as_str(), "haiku.txt");
    }

    #[test]
    fn working_args2() {
        let args = vec![String::from("./hexdump"), String::from("haiku.txt"), String::from("hello")];
        let args = Args::parse(&args).unwrap();
        assert_eq!(args.len, None);
        assert_eq!(args.file_path.as_str(), "haiku.txt");
    }

    #[test]
    fn working_args_with_optional_flag() {
        let args = vec![String::from("./hexdump"), String::from("-n"), String::from("1"), String::from("haiku.txt")];
        let args = Args::parse(&args).unwrap();
        assert_eq!(args.file_path.as_str(), "haiku.txt");
        assert_eq!(1, args.len.unwrap());
    }

    #[test]
    fn working_args_with_optional_flag2() {
        let args = vec![String::from("./hexdump"), String::from("haiku.txt"), String::from("-n"), String::from("1")];
        let args = Args::parse(&args).unwrap();
        assert_eq!(args.file_path.as_str(), "haiku.txt");
        assert_eq!(1, args.len.unwrap());
    }

    #[test]
    fn working_args_with_optional_flag3() {
        let args = vec![String::from("./hexdump"), String::from("haiku.txt"), String::from("-n"), String::from("1"), String::from("aaaaa")];
        let args = Args::parse(&args).unwrap();
        assert_eq!(args.file_path.as_str(), "haiku.txt");
        assert_eq!(1, args.len.unwrap());
    }

    #[test]
    fn broken_args_with_flag() {
        let args = vec![String::from("./hexdump"), String::from("haiku.txt"), String::from("-n")];
        let args = Args::parse(&args);
        match args {
            Err(e) => {
                assert_eq!(e.as_str(), "Usage: ./hexdump [-n LEN] FILE");
            }
            _ => {
                panic!();
            }
        }
    }

    #[test]
    fn broken_args_with_flag2() {
        let args = vec![String::from("./hexdump"), String::from("haiku.txt"), String::from("-n"), String::from("r")];
        let args = Args::parse(&args);
        match args {
            Err(e) => {
                assert_eq!(e.as_str(), "Invalid length: r\nUsage: ./hexdump [-n LEN] FILE");
            }
            _ => {
                panic!();
            }
        }
    }

    #[test]
    fn broken_args_with_flag3() {
        let args = vec![String::from("./hexdump"), String::from("-n"), String::from("r")];
        let args = Args::parse(&args);
        match args {
            Err(e) => {
                assert_eq!(e.as_str(), "Invalid length: r\nUsage: ./hexdump [-n LEN] FILE");
            }
            _ => {
                panic!();
            }
        }
    }

    #[test]
    fn args_with_no_file() {
        let args = vec![String::from("./hexdump"), String::from("-n"), String::from("4")];
        let args = Args::parse(&args);
        match args {
            Err(e) => {
                assert_eq!(e.as_str(), "Usage: ./hexdump [-n LEN] FILE");
            }
            _ => {
                panic!();
            }
        }
    }
}
