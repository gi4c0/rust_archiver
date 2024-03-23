use derive_more::AsRef;
use parse_display::Display;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::hash::Hash;

#[derive(
    PartialEq, Eq, Clone, Debug, FromRow, sqlx::Type, Deserialize, Serialize, Display, AsRef,
)]
#[sqlx(transparent)]
pub struct BetID(pub String);

#[derive(
    PartialEq, Eq, Clone, Debug, FromRow, sqlx::Type, Deserialize, Serialize, Display, AsRef,
)]
#[sqlx(transparent)]
pub struct ProviderBetID(pub String);

#[derive(PartialEq, Eq, Clone, Debug, FromRow, sqlx::Type, Deserialize, Serialize, Display)]
#[sqlx(transparent)]
pub struct ProviderGameVendorID(pub String);

// Need hash for storing in Map/Set
impl Hash for ProviderGameVendorID {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

#[derive(PartialEq, Eq, Clone, Debug, FromRow, sqlx::Type, Deserialize, Serialize, Display)]
#[sqlx(transparent)]
pub struct ProviderGameLabel(pub String);
