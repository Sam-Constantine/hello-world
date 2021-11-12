use std::time::{Duration, Instant};

use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_files as fs;
use actix_web::{App, get, HttpResponse, HttpServer, middleware, post, Responder, web};
use actix_web::{Error, HttpRequest};
use actix_web_actors::ws;

use module::user;

mod module;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            //.service(hello)
            .route("/hey", web::get().to(user::api::manual_hello))
            // websocket route
            .service(web::resource("/ws/").route(web::get().to(module::ws::ws_index)))
            // static files
            .service(fs::Files::new("/", "static/").index_file("ws.html"))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
//         .bind("127.0.0.1:8080")?
//         .run()
//         .await
// }
