use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub fullname: String,
    pub username: String,
    pub email: String,
    pub number: String,
    pub password_hash: String,
    pub salt: String,
}
