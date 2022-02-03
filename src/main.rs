#[macro_use]
extern crate lazy_static;
extern crate argon2;
extern crate dotenv;
extern crate env_logger;

use std::{path::Path, sync::{Arc, Mutex}};

use actix_files::{Files, NamedFile};
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, get};
use dotenv::dotenv;

mod logic; 
use logic::*;
mod routes;
use routes::*;

#[get("/favicon.ico")]
async fn favicon() -> std::io::Result<NamedFile> {
    NamedFile::open(Path::new("static/favicon.ico"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = std::env::var("PORT").unwrap();
    
    println!("Starting server 0.0.0.0:{}...", port);

    std::env::set_var("RUST_LOG", "actix_web=error");
    env_logger::init();

    let app_state = Arc::new(Mutex::new(AppState::new()?));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(Arc::clone(&app_state))
            .service(index)
            .service(favicon)
            .service(register_get)
            .service(register_post)
            .service(login_get)
            .service(login_post)
            .service(logout)
            .service(paste)
            .service(paste_upload)
            .service(Files::new("/js", Path::new("public/js")).prefer_utf8(true))
            .service(Files::new("/css", Path::new("public/css")).prefer_utf8(true))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
