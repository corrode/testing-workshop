//! Sample use-case, which uses the repository

use std::fmt::{self, Display, Formatter};

use crate::repository::{Repository, User};
use crate::Result;

#[derive(Debug, PartialEq)]
pub struct UserStatistics {
    /// Total number of users.
    total: usize,
}

impl Display for UserStatistics {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Total users: {}", self.total)
    }
}

impl UserStatistics {
    /// Calculate the total number of users.
    pub fn calculate_total(repository: &Box<dyn Repository<User>>) -> Result<Self> {
        let users = repository.find_all()?;
        let total = users.len();

        Ok(UserStatistics { total })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{repository::fake_user_repository::FakeUserRepository, UserRepository};

    #[test]
    fn test_real_user_statistics() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL
                  )",
            [],
        )
        .unwrap();
        conn.execute("INSERT INTO person (name) VALUES (?1)", &[&"Alice"])
            .unwrap();
        conn.execute("INSERT INTO person (name) VALUES (?1)", &[&"Bob"])
            .unwrap();

        let repository = Box::new(UserRepository::new(conn)) as Box<dyn Repository<User>>;
        let stats = UserStatistics::calculate_total(&repository).unwrap();

        assert_eq!(stats.total, 2);
    }

    #[test]
    fn test_user_statistics() {
        let repository = Box::new(FakeUserRepository::new()) as Box<dyn Repository<User>>;
        let stats = UserStatistics::calculate_total(&repository).unwrap();

        assert_eq!(stats.total, 2);
    }
}
