use std::env;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref MARIA_DB_SCHEMA: String =
        env::var("MARIA_DB_SCHEMA").unwrap_or("public".to_string());
    pub static ref SCHEMA: String = env::var("SCHEMA").unwrap_or("public".to_string());
}
