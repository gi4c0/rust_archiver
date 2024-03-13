mod bet;
mod user;

pub use bet::*;
use parse_display::Display;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
pub use user::*;

pub type AmountByPosition = [i64; 7];

#[derive(PartialEq, Eq, Clone, Debug, FromRow, sqlx::Type, Deserialize, Serialize, Display)]
#[sqlx(transparent)]
#[display("{0}")]
pub struct Url(pub String);
