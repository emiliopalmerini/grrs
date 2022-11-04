#![allow(unused)]

use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    
    // read_to_string returns Result with an Error. Result is an enum 
    // .expect will be improved in future
    let content = std::fs::read_to_string(&args.path)?;
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }    
    Ok(())
}
