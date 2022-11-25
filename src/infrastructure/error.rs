use std::io;
use std::io::Error;
use rodio::StreamError;
use crate::domain::error::DomainError;

impl From<rodio::decoder::DecoderError> for DomainError {
    fn from(error: rodio::decoder::DecoderError) -> Self {
        DomainError::Resource(error.to_string())
    }
}

impl From<io::Error> for DomainError {
    fn from(error: Error) -> Self {
        DomainError::Resource(error.to_string())
    }
}

impl From<rodio::StreamError> for DomainError {
    fn from(error: StreamError) -> Self {
        DomainError::EquipmentError(error.to_string())
    }
}

impl From<rodio::PlayError> for DomainError {
    fn from(error: rodio::PlayError) -> Self {
        DomainError::EquipmentError(error.to_string())
    }
}
