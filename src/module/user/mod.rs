pub mod api {
    use std::fs;
    use std::future::Future;
    use std::io::Write;

    use actix_form_data::Value;
    use actix_multipart::{Field, MultipartError};
    use actix_web::{Error, error, get, HttpResponse, post, Responder, web};
    use actix_web::dev::Service;

    pub async fn manual_hello() -> impl Responder {
        HttpResponse::Ok().body("Hey there!")
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
"#, std::env::current_exe().unwrap().as_path().to_str().unwrap());

        HttpResponse::Ok().body(ret)
    }

    #[post("/echo")]
    pub async fn echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
    }
}
