

use actix_web::{Error, get, HttpRequest, HttpResponse, post, Responder, web};
use log::{debug, info};
use actix_web_actors::ws;
use crate::ws_conn::WsConn;

pub async fn manual_hello() -> impl Responder {
    let ret = HttpResponse::Ok().body("Hey there! " );
    debug!("{:?}", &ret);
    ret
}

#[get("/")]
pub async fn index() -> impl Responder {
    let ret = format!(
r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Server Server Server</title>
    </head>
    <body>
        <h1>Hi from Server</h1>
        <p>运行于： {}</p>
    </body>
    </html>
"#,
        std::env::current_exe().unwrap().as_path().to_str().unwrap()
    );

    HttpResponse::Ok().body(ret)
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

/// do websocket handshake and start `MyWebSocket` actor
#[get("/ws/chat")]
pub async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    info!("Receive=> \n{:?}", r);
    let res = ws::start(WsConn::new(), &r, stream);
    info!("Result=> \n{:?}", res);
    res
}
