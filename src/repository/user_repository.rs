use super::{Repository, User};
use crate::Result;

/// User repository
pub struct UserRepository {
    conn: rusqlite::Connection,
}

impl UserRepository {
    /// Create a new user repository.
    pub fn new(conn: rusqlite::Connection) -> Self {
        UserRepository { conn }
    }
}

impl Repository<User> for UserRepository {
    fn find_all(&self) -> Result<Vec<User>> {
        let mut stmt = self.conn.prepare("SELECT id, name FROM person")?;
        let person_iter = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        });

        let mut users = Vec::new();
        for person in person_iter? {
            users.push(person?);
        }
        Ok(users)
    }

    fn find_by_id(&self, id: i32) -> Result<Option<User>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name FROM person WHERE id = ?1")?;

        let users_iter = stmt.query_map([id], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        });

        let ret = if let Some(user) = users_iter?.next() {
            Ok(Some(user?))
        } else {
            Ok(None)
        };

        ret
    }
}

/// Real tests against the sqlite database.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all() {
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

        let repository = UserRepository::new(conn);
        let users = repository.find_all().unwrap();
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].name, "Alice");
        assert_eq!(users[1].name, "Bob");
    }

    #[test]
    fn test_find_by_id() {
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

        let repository = UserRepository::new(conn);
        let user = repository.find_by_id(1).unwrap().unwrap();
        assert_eq!(user.name, "Alice");
    }
}
