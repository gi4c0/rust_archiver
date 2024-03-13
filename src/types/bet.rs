use parse_display::Display;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(PartialEq, Eq, Clone, Debug, FromRow, sqlx::Type, Deserialize, Serialize, Display)]
#[sqlx(transparent)]
pub struct BetID(pub String);

#[derive(PartialEq, Eq, Clone, Debug, FromRow, sqlx::Type, Deserialize, Serialize, Display)]
#[sqlx(transparent)]
pub struct ProviderBetID(pub String);

#[derive(PartialEq, Eq, Clone, Debug, FromRow, sqlx::Type, Deserialize, Serialize, Display)]
#[sqlx(transparent)]
pub struct ProviderGameVendorID(pub String);
