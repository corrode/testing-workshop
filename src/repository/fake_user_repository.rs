#[cfg(test)]
use super::{Repository, User};
#[cfg(test)]
use crate::Result;

/// Fake repository for the `User` entity.
#[cfg(test)]
pub struct FakeUserRepository;

#[cfg(test)]
impl FakeUserRepository {
    /// Create a new fake user repository.
    pub fn new() -> Self {
        FakeUserRepository
    }
}

#[cfg(test)]
impl Repository<User> for FakeUserRepository {
    fn find_all(&self) -> Result<Vec<User>> {
        Ok(vec![
            User {
                id: 1,
                name: "Alice".to_string(),
            },
            User {
                id: 2,
                name: "Bob".to_string(),
            },
        ])
    }

    fn find_by_id(&self, id: i32) -> Result<Option<User>> {
        match id {
            1 => Ok(Some(User {
                id: 1,
                name: "Alice".to_string(),
            })),
            2 => Ok(Some(User {
                id: 2,
                name: "Bob".to_string(),
            })),
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all() {
        let repository = FakeUserRepository;
        let users = repository.find_all().unwrap();
        assert_eq!(users.len(), 2);
    }

    #[test]
    fn test_find_by_id() {
        let repository = FakeUserRepository;
        let user = repository.find_by_id(1).unwrap();
        assert_eq!(user.unwrap().name, "Alice");
    }
}
