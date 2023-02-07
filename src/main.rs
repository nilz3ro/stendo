use clap::Parser;
use stendo::{
    cli::Cli,
    error::StendoError,
    process::{
        copy_image_file, get_metadata_from_file, overwrite_metadata_with_index,
        write_metadata_to_json_file,
    },
};

fn main() -> Result<(), StendoError> {
    let cli = Cli::parse();

    let metadata = get_metadata_from_file(&cli.metadata_file)?;

    for i in 0..cli.num {
        let new_metadata = overwrite_metadata_with_index(&metadata, i as usize)?;
        write_metadata_to_json_file(&new_metadata, &cli.out_dir, i as usize)?;
        copy_image_file(&cli.image_file, &cli.out_dir, i as usize)?;
    }

    Ok(())
}
