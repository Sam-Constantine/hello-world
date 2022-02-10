use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};

mod module;
use module::controller;
use module::controller::ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(controller::api::index)
            .service(controller::api::echo)
            .route("/hey", web::get().to(controller::api::manual_hello))
            // websocket route
            //.service(web::resource("/ws/chat").route(web::get().to(ws::ws_index)))
            .service(ws::ws_index)
            // static files
            .service(fs::Files::new("/", "static/").index_file("ws.html"))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
