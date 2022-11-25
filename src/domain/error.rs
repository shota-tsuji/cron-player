use thiserror::Error;
use std::{fmt, io};
use std::fmt::Formatter;

#[derive(Debug, Error)]
pub enum ProcessError {
    #[error("{0}")]
    Validation(String),
    #[error(transparent)]
    Resource(anyhow::Error),
    #[error(transparent)]
    EquipmentError(anyhow::Error),
}
