use actix_web::{web, HttpResponse, Responder};
use argon2rs::argon2i_simple;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
// db model
#[path = "../../src/data/user_model.rs"]
mod user_model;
use user_model::User;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
pub async fn login(login_data: web::Json<LoginRequest>) -> Result<impl Responder, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path("./src/data/data.csv")?;
    for result in rdr.deserialize() {
        let user: User = result?;

        if user.username == login_data.username {
            let attempt_hash = argon2i_simple(&login_data.password, "salt");
            if user.password_hash == attempt_hash {
                return Ok(HttpResponse::Ok().body("Giriş başarılı"));
            }
        }
    }
    Ok(HttpResponse::Unauthorized().body("Geçersiz kullanıcı adı veya şifre"))
}
