use std::hash::Hash;

use parse_display::Display;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(
    PartialEq, Eq, Clone, Debug, FromRow, sqlx::Type, Deserialize, Serialize, Display, Copy,
)]
#[sqlx(transparent)]
pub struct UserID(pub Uuid);

#[derive(PartialEq, Eq, Clone, Debug, FromRow, sqlx::Type, Deserialize, Serialize, Display)]
#[sqlx(transparent)]
pub struct Username(pub String);

impl From<String> for Username {
    fn from(value: String) -> Self {
        Username(value)
    }
}

// Need hash for storing in Map/Set
impl Hash for UserID {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

// For converting uuid from DB to our type
impl From<Uuid> for UserID {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

#[derive(PartialEq, Eq, Clone, Debug, FromRow, sqlx::Type, Deserialize, Serialize)]
#[sqlx(transparent)]
pub struct Currency(pub String);

pub type Upline = [Option<UserID>; 7];
