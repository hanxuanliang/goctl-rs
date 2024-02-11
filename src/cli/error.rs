#![allow(dead_code)]

use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransformError {
    #[error("Unsupported command")]
    UnsupportedCommand,

    #[error("Failed to read file {0}")]
    IOError(#[from] std::io::Error),

    #[error("Failed to parse at {0}")]
    ParseError(String),

    #[error("Failed to create output directory {0}")]
    OutDirError(PathBuf),

    #[error("Failed to create file path at: {path}")]
    CreateFileError {
        path: String,
        source: std::io::Error,
    },

    #[error("Failed to write to file at: {path}")]
    WriteFileError {
        path: String,
        source: std::io::Error,
    },
}
