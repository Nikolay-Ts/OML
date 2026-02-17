use std::fmt;
use crate::define_error;

define_error!(NameError, "Name Error:");

#[derive(Debug)]
pub enum ParseError {
    Io(),
    MaxDepthExceeded,
    InvalidPath,
}

impl From<std::io::Error> for ParseError {
    fn from(_: std::io::Error) -> Self {
        ParseError::Io()
    }
}