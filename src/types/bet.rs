use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(PartialEq, Eq, Clone, Debug, FromRow, sqlx::Type, Deserialize, Serialize)]
#[sqlx(transparent)]
pub struct BetID(pub String);
