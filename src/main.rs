pub mod lexer;
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

    println!("{}", args.filename);
    Ok(())
}
