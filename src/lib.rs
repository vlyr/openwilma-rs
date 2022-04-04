use std::error::Error as StdError;
use std::fmt;

pub mod client;
pub use client::{Client, Credentials};

/// The Error enum. Used for handling Wilma-specific errors.
#[derive(Debug)]
pub enum Error {
    InvalidCredentials,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        let msg = match self {
            InvalidCredentials => "Invalid credentials were provided.",
        };

        write!(f, "{}", msg)
    }
}

impl StdError for Error {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
