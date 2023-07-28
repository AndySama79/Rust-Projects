#![allow(unused)]

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]

struct Cli {
    /// The patter to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,   // PathBuf is like a String but for file system paths that work cross-platform
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
