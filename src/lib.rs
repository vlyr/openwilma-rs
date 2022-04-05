use std::env;
use std::error::Error as StdError;
use std::fmt;

pub mod client;
pub use client::{Client, Credentials};

pub mod parser;
pub mod utils;
pub mod wilma;

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
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn login() {
        let credentials = Credentials {
            username: &env::var("USERNAME").unwrap(),
            password: &env::var("PASSWORD").unwrap(),
            server: &env::var("SERVER").unwrap(),
        };

        let client = Client::login(credentials).await.unwrap();

        println!("{:#?}", client.get_user_profile().await.unwrap());
    }

    #[tokio::test]
    async fn schedule() {
        let credentials = Credentials {
            username: &env::var("USERNAME").unwrap(),
            password: &env::var("PASSWORD").unwrap(),
            server: &env::var("SERVER").unwrap(),
        };

        let client = Client::login(credentials).await.unwrap();

        client.get_user_schedule().await.unwrap();
    }

    #[tokio::test]
    async fn overview() {
        let credentials = Credentials {
            username: &env::var("USERNAME").unwrap(),
            password: &env::var("PASSWORD").unwrap(),
            server: &env::var("SERVER").unwrap(),
        };

        let client = Client::login(credentials).await.unwrap();

        let overview = client.get_overview().await.unwrap();

        println!("{:#?}", overview);
    }
}
