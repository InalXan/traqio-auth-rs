use actix_web::{web, HttpResponse, Responder};
use argon2rs::argon2i_simple;
use csv::WriterBuilder;
use std::error::Error;
// user model
#[path = "../../src/data/user_model.rs"]
mod user_model;
use user_model::User;

pub async fn register(user: web::Json<User>) -> Result<impl Responder, Box<dyn Error>> {
    let password_hash = argon2i_simple(&user.password, "salt");

    let user_with_hash = User {
        password_hash,
        ..user.into_inner()
    };

    let mut wrt = WriterBuilder::new().from_path("./src/data/data.csv")?;
    wrt.serialize(&user_with_hash)?;
    wrt.flush()?;

    Ok(HttpResponse::Ok().body("Kullanıcı kaydedildi"))
}
