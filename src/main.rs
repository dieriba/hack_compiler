mod keywords;
pub mod lexer;
use std::fs;

use anyhow::Result;
use clap::Parser;
use lexer::Lexer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file = fs::read_to_string(args.filename)?;
    let lexer = Lexer::new(&file);
    for token in lexer {
        if let Ok(token) = token {
            println!("{:#?}", token);
        }
    }
    Ok(())
}
