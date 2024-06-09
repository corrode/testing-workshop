mod fake_user_repository;
pub mod user_repository;

use crate::Result;

/// Repository trait.
///
/// This trait defines the basic operations that a repository should implement.
trait Repository<T> {
    /// Get all entities from the repository.
    fn find_all(&self) -> Result<Vec<T>>;
    /// Find an entity by its id.
    fn find_by_id(&self, id: i32) -> Result<Option<T>>;
}

/// Sample user entity.
pub struct User {
    /// User id.
    pub id: i32,
    /// User name.
    pub name: String,
}
