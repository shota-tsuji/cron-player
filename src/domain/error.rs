use thiserror::Error;
use std::{fmt, io};
use std::fmt::Formatter;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("{0}")]
    Validation(String),
    #[error("{0}")]
    Resource(String),
    #[error("{0}")]
    EquipmentError(String),
}
