use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to render: {0}")]
    RenderError(#[from] askama::Error),
    #[error("Unable to write: {0}")]
    IOError(#[from] io::Error),
    #[error("Unknown error")]
    Unknown,
}
