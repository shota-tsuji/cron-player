use std::io;
use std::io::Error;
use rodio::StreamError;
use crate::domain::error::ProcessError;

impl From<rodio::decoder::DecoderError> for ProcessError {
    fn from(error: rodio::decoder::DecoderError) -> Self {
        ProcessError::Resource(anyhow::Error::new(error))
    }
}

impl From<io::Error> for ProcessError {
    fn from(error: Error) -> Self {
        ProcessError::Resource(anyhow::Error::new(error))
    }
}

impl From<rodio::StreamError> for ProcessError {
    fn from(error: StreamError) -> Self {
        ProcessError::EquipmentError(anyhow::Error::new(error))
    }
}

impl From<rodio::PlayError> for ProcessError {
    fn from(error: rodio::PlayError) -> Self {
        ProcessError::EquipmentError(anyhow::Error::new(error))
    }
}
