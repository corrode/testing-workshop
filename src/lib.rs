use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

pub mod domain;

/// Part 1 about unit testing.
pub mod fibonacci;
pub mod prime;

/// Part 2 about integration testing.
mod repository;

/// Exercises for learning about testing.
mod exercises;

pub use repository::user_repository::UserRepository;

#[derive(Debug, PartialEq)]
pub enum TestError {
    Repository(rusqlite::Error),
}

impl Display for TestError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TestError")
    }
}

impl Error for TestError {}

impl From<rusqlite::Error> for TestError {
    fn from(e: rusqlite::Error) -> Self {
        TestError::Repository(e)
    }
}

type Result<T> = std::result::Result<T, TestError>;
