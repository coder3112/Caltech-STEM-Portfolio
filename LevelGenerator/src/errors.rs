use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct LevelTooBigError {
    pub message: String,
}

impl LevelTooBigError {
    pub fn new(msg: String) -> Self {
        LevelTooBigError {
            message: msg.to_string(),
        }
    }
}

impl std::fmt::Display for LevelTooBigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for LevelTooBigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
