//! CLI for binaryornot — check if a file is binary or text.
//!
//! Usage:
//! ```sh
//! binaryornot <file>
//! binaryornot <file1> <file2> ...
//! ```

use std::path::Path;
use std::process;

use binaryornot_rs::check::is_binary;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: binaryornot <file> [file2] ...");
        eprintln!("Check if one or more files are binary or text.");
        process::exit(1);
    }

    let mut had_error = false;
    for arg in &args {
        let path = Path::new(arg);
        match is_binary(path, true) {
            Ok(result) => println!("{}", result),
            Err(e) => {
                eprintln!("Error reading '{}': {}", arg, e);
                had_error = true;
            }
        }
    }

    if had_error {
        process::exit(1);
    }
}
