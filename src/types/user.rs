use std::hash::Hash;

use uuid::Uuid;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UserID(pub Uuid);

impl Hash for UserID {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl From<Uuid> for UserID {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}
