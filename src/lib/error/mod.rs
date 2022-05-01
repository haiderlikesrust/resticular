use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Parser Error: {0}")]
    ParserError(String),
    #[error("File I/O: {0}")]
    FileIOError(#[from] std::io::Error),
    #[error("{0}")]
    HtmlMinifyError(String)

}
