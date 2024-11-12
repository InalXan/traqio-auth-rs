use actix_web::{App, HttpServer};
// routerin
#[path = "../src/router/index.rs"]
mod index;
use index::configure_router;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // host configure
    let host = "127.0.0.1:8080";
    let host_err = "Server Not Working";

    HttpServer::new(move || App::new().configure(configure_router))
        .bind(host)
        .expect(host_err)
        .workers(2)
        .run()
        .await
}
