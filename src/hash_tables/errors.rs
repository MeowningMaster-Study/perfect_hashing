use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NonUniqueError {}

impl fmt::Display for NonUniqueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Passed non-unique data to hash table!")
    }
}

impl Error for NonUniqueError {}
