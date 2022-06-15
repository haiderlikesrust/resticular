use thiserror::Error;
use toml::de::Error as DeError;
#[derive(Debug, Error)]
pub enum Error {
    #[error("Parser Error: {0}")]
    ParserError(String),
    #[error("File I/O: {0}")]
    FileIOError(#[from] std::io::Error),
    #[error("{0}")]
    HtmlMinifyError(String),
    #[error("Condition failed")]
    PageCheckError,
    #[error("Html rewriting error {0}")]
    HtmlRewriteError(#[from] lol_html::errors::RewritingError),
    #[error("{0} failed while doing {1}")]
    ThreadFailed(String, String),
    #[error("{0}")]
    FileSystemWatchError(#[from] hotwatch::Error),
    #[error("Error while sending message from a thread.")]
    MsgError,
    #[error("Error while receiving message from a thread.")]
    RecvError,
    #[error("Fs Error: {0}")]
    FsError(#[from] fs_extra::error::Error),
    #[error("Config file error: {0}")]
    ConfigFileError(#[from] ConfigError),
    #[error("Image Error: {0}")]
    ImageError(#[from] image::ImageError)
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("{0}")]
    ConfigFileNotFound(String),
    #[error("Required Field: {0}")]
    ConfigFileRequiredField(#[from] DeError),
    #[error("Parse Error: {0}")]
    ConfigFileParseError(String)
}