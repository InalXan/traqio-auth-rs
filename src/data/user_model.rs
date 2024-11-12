use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub fullname: String,
    pub username: String,
    pub number: String,
    pub email: String,
    pub password: String,
}
