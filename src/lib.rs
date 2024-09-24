use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

const BYTES_PER_LINE: usize = 16;

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let file = File::open(args.file_path)?;
    
    if let Some(n) = args.len {
        read_convert_print(file.take(n))?;
    } else {
        read_convert_print(file)?;
    }

    Ok(())
}

fn read_convert_print<R: Read>(file: R) -> Result<(), Box<dyn Error>>{
    let mut reader = BufReader::new(file);
    let mut buf: [u8; BYTES_PER_LINE] = [0; BYTES_PER_LINE];
    
    let mut offset = 0usize;
    let mut n = reader.read(&mut buf)?;
    while n != 0 {
        print_hexdump_line(&buf, n, offset);
        offset += n;
        n = reader.read(&mut buf)?;
    }

    println!("{:08x}", offset);

    Ok(())
}

fn print_hexdump_line(buf: &[u8], n: usize, offset: usize) {
    let mut line = Vec::new();
    line.push(format!("{:08x}", offset));

    for i in (0..BYTES_PER_LINE).step_by(2) {
        if i + 1 < n {
            let in_hex = (buf[i + 1] as u16) << 8 | buf[i] as u16;
            line.push(format!("{in_hex:04x}"));
        } else if i < n {
            line.push(format!("{:04x}", buf[i]));
        } else {
            line.push(String::from("    "));
        }
    }

    println!("{}", line.join(" "));
}

pub struct Args {
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

        Ok(Args { len, file_path })
    }
}

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
