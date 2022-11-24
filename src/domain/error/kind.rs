use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ProcessError {
    FileReadError,
    OutputStreamError,
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ProcessError::FileReadError => write!(f, "file state is invalid"),
            ProcessError::OutputStreamError => write!(f, "output stream setting is failed"),
        }
    }
}
