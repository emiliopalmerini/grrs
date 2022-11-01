#![allow(unused)]

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();
    println!("pattern to find: {}", args.pattern);
    println!("path where to search: {}", args.path.display());
}