use std::fmt;


#[derive(Debug, Clone, PartialEq)]
pub struct NameError {
    pub message: String
}

impl NameError {
    pub fn new(message: String) -> Self {
        Self {
            message
        }
    }
}

impl fmt::Display for NameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// AND THIS:
impl std::error::Error for NameError {}