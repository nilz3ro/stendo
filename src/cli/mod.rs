use clap::Parser;

#[derive(Parser)]
#[command(name = "stendo")]
pub struct Cli {
    pub metadata_file: String,
    pub image_file: String,
    #[arg(short = 'n', long, default_value = "10")]
    pub num: u32,
    #[arg(short = 'o', long, default_value = "./assets")]
    pub out_dir: String,
}
