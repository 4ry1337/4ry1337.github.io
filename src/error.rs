use std::io;

use thiserror::Error;

use crate::utils::error_chain_fmt;

#[derive(Error)]
pub enum SsgError {
    #[error("Unable to render")]
    RenderError(
        #[from]
        #[source]
        askama::Error,
    ),
    #[error("Unable to write")]
    IOError(
        #[from]
        #[source]
        io::Error,
    ),
    #[error("Unable to parse")]
    SerderError(
        #[from]
        #[source]
        serde_yaml::Error,
    ),
    #[error("Unknown error")]
    Unknown,
}

impl std::fmt::Debug for SsgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

// impl From<std::io::Error> for SsgError {
//     fn from(value: std::io::Error) -> Self {
//         Self::IOError(value)
//     }
// }
//
// impl From<askama::Error> for SsgError {
//     fn from(value: askama::Error) -> Self {
//         Self::RenderError(value)
//     }
// }
//
// impl From<serde_yaml::Error> for SsgError {
//     fn from(value: serde_yaml::Error) -> Self {
//         Self::SerderError(value)
//     }
// }
