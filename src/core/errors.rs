use std::fmt;
use crate::define_error;

define_error!(NameError, "Name Error:");
define_error!(ConstError, "Modifier Error: ");


#[derive(Debug)]
pub enum ParseError {
    Io(std::io::Error),
    MaxDepthExceeded,
    InvalidPath,
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::Io(err)
    }
}