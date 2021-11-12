use std::time::{Duration, Instant};

use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_files as fs;
use actix_form_data::{Field, Form, Value};
use actix_web::{App, get, HttpResponse, HttpServer, middleware, post, Responder, web};
use actix_web::{Error, HttpRequest};
use actix_web::web::{post, resource};
use actix_web_actors::ws;
use futures::StreamExt;

use module::user;

mod module;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(user::api::index)
            .service(user::api::echo)
            .route("/hey", web::get().to(user::api::manual_hello))
        // websocket route
        // .service(web::resource("/ws/chat").route(web::get().to(module::ws::ws_index)))
        // static files
        // .service(fs::Files::new("/", "static/").index_file("ws.html"))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

