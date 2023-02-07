use thiserror::Error;

#[derive(Error, Debug)]
pub enum StendoError {
    #[error("Failed to process file")]
    FileError(#[from] std::io::Error),
    #[error("Failed to (de)serialize JSON")]
    SerdeError(#[from] serde_json::Error),
}
