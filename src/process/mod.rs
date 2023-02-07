use crate::{cli::Cli, error::StendoError};
use serde_json;
use std::{ffi::OsStr, fs::File, io, path::Path};
use sugar_cli::validate::Metadata;

pub fn get_metadata_from_file(file_path: &str) -> Result<Metadata, StendoError> {
    let file = File::open(file_path)?;
    let metadata: Metadata = serde_json::from_reader(file)?;

    Ok(metadata)
}

pub fn overwrite_metadata_with_index(
    metadata: &Metadata,
    index: usize,
) -> Result<Metadata, StendoError> {
    let mut cloned_metadata = metadata.clone();

    cloned_metadata.name = str::replace(&cloned_metadata.name, "{n}", &((index + 1).to_string()));
    cloned_metadata.image = str::replace(&cloned_metadata.image, "{n}", &index.to_string());

    for f in cloned_metadata.properties.files.iter_mut() {
        f.uri = str::replace(&f.uri, "{n}", &index.to_string());
    }

    Ok(cloned_metadata)
}

pub fn write_metadata_to_json_file(
    cli: &Cli,
    metadata: &Metadata,
    index: usize,
) -> Result<(), StendoError> {
    let file_name = format!("{}/{}.json", cli.out_dir, index);
    let file = File::create(file_name)?;

    serde_json::to_writer_pretty(file, metadata)?;

    Ok(())
}

pub fn copy_image_file(cli: &Cli, index: usize) -> Result<(), StendoError> {
    let mut src_file = File::open(&cli.image_file)?;

    let extension = Path::new(&cli.image_file)
        .extension()
        .and_then(OsStr::to_str)
        .expect("should have an extension");

    let dst_file_name = format!("{}/{}.{}", cli.out_dir, index, extension);
    let mut dst_file = File::create(dst_file_name)?;

    io::copy(&mut src_file, &mut dst_file)?;

    Ok(())
}
