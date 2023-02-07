use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fs, path::Path, thread::Scope};

#[derive(Parser)]
#[command(name = "stendo")]
struct Cli {
    #[arg(short = 'n', long)]
    num: u32,
}

fn main() {
    let cli = Cli::parse();

    println!("num: {}", cli.num);
}
