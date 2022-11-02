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
    // println!("pattern to find: {}", args.pattern);
    // println!("path where to search: {}", args.path.display());
    let content = std::fs::read_to_string(&args.path).expect("couldn't read file")?;
    };

    // .expect will be improved in future
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }    
    Ok(())
}
