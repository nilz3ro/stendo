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

    // TODO: parallelize this
    for i in cli.offset..cli.num + cli.offset {
        let new_metadata = overwrite_metadata_with_index(&metadata, i as usize)?;

        write_metadata_to_json_file(&cli, &new_metadata, i as usize)?;
        copy_image_file(&cli, i as usize)?;
    }

    Ok(())
}
