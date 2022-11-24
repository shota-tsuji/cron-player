use thiserror::Error;
use std::{fmt, io};
use std::fmt::Formatter;

#[derive(Debug, Error)]
pub enum ProcessError {
    #[error("file state is invalid")]
    FileReadError(#[from] io::Error),
    #[error("output stream setting is failed")]
    OutputStreamError(#[from] rodio::StreamError),
    #[error("decoding is failed")]
    DecodeError(#[from] rodio::decoder::DecoderError),
    #[error("sound playing is failed")]
    PlayingError(#[from] rodio::PlayError),
}
