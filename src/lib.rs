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
    let mut buf = [0u8; BYTES_PER_LINE];
    
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
pub fn convert_to_hexdump_line(buf: &[u8], n: usize) -> String {
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
    fn usage_message(program_name: &String) -> String {
        format!("Usage: {program_name} [-n LEN] FILE")
    }

    pub fn parse(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Args, String> {
        let program_name = match args.next() {
            Some(name) => name,
            None => {
                return Err(String::from("Error"));
            }
        };
        let mut len: Option<u64> = None;
        let mut file_path: Option<String> = None;

        // iterate through command line arguments to check for optional flags
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-n" => {
                    let next_arg = match args.next() {
                        Some(s) => s,
                        _ => {
                            return Err(Self::usage_message(&program_name));
                        }
                    };
                    len = match next_arg.parse() {
                        Ok(len) => Some(len),
                        _ => {
                            return Err(format!("Invalid length: {}\n{}",
                                               next_arg,
                                               Self::usage_message(&program_name)));
                        }
                    };
                }
                _ => {
                    if file_path.is_none() {
                        file_path = Some(arg)
                    }
                }
            }
        }

        // make sure there was a filename in the input
        let file_path = match file_path {
            Some(file) => file,
            _ => {
                return Err(Self::usage_message(&program_name));
            }
        };

        Ok(Args { program_name, len, file_path })
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_hex() {
        let input = [0xffu8; BYTES_PER_LINE];
        let n = 16usize;
        let result = String::from("ffff ffff ffff ffff ffff ffff ffff ffff");
        assert_eq!(result, convert_to_hexdump_line(&input, n));
    }

    #[test]
    fn hex_convert1() {
        let input = [0xffu8; BYTES_PER_LINE];
        let n = 5usize;
        let result = String::from("ffff ffff 00ff                         ");
        assert_eq!(result, convert_to_hexdump_line(&input, n));
    }

    #[test]
    fn hex_with_0_len() {
        let input = [0xffu8; BYTES_PER_LINE];
        let n = 0usize;
        let result = String::from("                                       ");
        assert_eq!(result, convert_to_hexdump_line(&input, n));
    }

    #[test]
    fn little_endian_hex() {
        let input = [0xaau8, 0xbb, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let n = 16usize;
        let result = String::from("bbaa 0000 0000 0000 0000 0000 0000 0000");
        assert_eq!(result, convert_to_hexdump_line(&input, n));
    }
}
