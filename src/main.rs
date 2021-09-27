extern crate actix_cors;
extern crate actix_web;
extern crate dotenv;
extern crate env_logger;
extern crate juniper;
extern crate serde;
extern crate serde_json;
extern crate sqlx;

use actix_web::{middleware, web, App, HttpServer};

mod cors;
mod database;
mod gpql;

use cors::create_cors;
use database::create_pool;
use gpql::handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let pool = create_pool().await;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::default())
            .wrap(create_cors())
            .data(pool.clone())
            .configure(handler::register)
            .default_service(web::to(|| async { "404" }))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
