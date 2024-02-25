use uuid::Uuid;

pub struct UserID(pub Uuid);

impl From<Uuid> for UserID {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}
