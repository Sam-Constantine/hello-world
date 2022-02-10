

use actix_web::{get, post, HttpResponse, Responder};
use log::debug;

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
