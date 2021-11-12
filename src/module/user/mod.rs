pub mod api {
    use actix_web::{get, HttpResponse, post, Responder};

    #[get("/")]
    async fn hello() -> impl Responder {
        let ret = format!(
            r#"
        <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Hello!</title>
    </head>
    <body>
        <h1>运行于： {:?}</h1>
        <p>Hi from Rust</p>
    </body>
    </html>
    "#,
            std::env::current_exe().unwrap()
        );
        HttpResponse::Ok().body(ret)
    }

    #[post("/echo")]
    async fn echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
    }

    pub async fn manual_hello() -> impl Responder {
        HttpResponse::Ok().body("Hey there!")
    }
}
