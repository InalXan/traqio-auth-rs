use actix_web::web;
//controllers
#[path = "../../src/controllers/register_controller.rs"]
mod register_controller;
use register_controller::register;
#[path = "../../src/controllers/login_controller.rs"]
mod login_controller;
use login_controller::login;

pub fn configure_router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/register").route(web::post().to(register)));
    cfg.service(web::resource("/login").route(web::post().to(login)));
}
