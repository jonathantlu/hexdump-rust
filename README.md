# hexdump
Simple Rust implementation of the hexdump utility.\
This utility only prints in little endian of a word size of 4 bytes.\
Usage: ``./hexdump [-n LEN] FILE``\
Supported Flags:
- ``-n LEN``: prints up to n bytes in hexadecimal
## Compiling:
With Rust installed, use ``cargo build --release``\
Instructions on installing Rust can be found here:
- https://doc.rust-lang.org/book/ch01-01-installation.html

Detailed compilation instructions can be found here:
- https://doc.rust-lang.org/book/ch01-03-hello-cargo.html#building-and-running-a-cargo-project
