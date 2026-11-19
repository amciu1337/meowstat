pub mod config;
pub mod routes;
pub mod monitor;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::{config::{Config, init_config}, routes::statistics::statistics};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: Config = init_config("./config.toml").expect("Failed to read config");
    HttpServer::new(|| {
        App::new()
            .service(routes::version::version)
            .service(routes::statistics::statistics)
    })
    .bind((config.ip, config.port))?
    .run()
    .await
}