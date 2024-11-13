use actix_web::{web, HttpResponse, Responder};
use argon2rs::argon2i_simple;
use csv::WriterBuilder;
use serde::Deserialize;
use std::error::Error;
use uuid::Uuid;
// user model
#[path = "../../src/data/user_model.rs"]
mod user_model;
use user_model::User;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub fullname: String,
    pub username: String,
    pub email: String,
    pub number: String,
    pub password: String,
}

pub async fn register(user: web::Json<RegisterRequest>) -> Result<impl Responder, Box<dyn Error>> {
    // create salt with uuid
    let salt = Uuid::new_v4().to_string();

    // hashing password
    let password_hash_bytes = argon2i_simple(&user.password, &salt);

    // hex formatter
    let password_hash = hex::encode(password_hash_bytes);

    // update salt
    let user_with_hash = User {
        fullname: user.fullname.clone(),
        username: user.username.clone(),
        email: user.email.clone(),
        number: user.number.clone(),
        password_hash,
        salt,
    };

    let mut wrt = WriterBuilder::new().from_path("./src/data/data.csv")?;
    wrt.serialize(&user_with_hash)?;
    wrt.flush()?;

    Ok(HttpResponse::Ok().body("user register successfully <3"))
}
