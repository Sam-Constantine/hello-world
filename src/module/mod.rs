pub mod controller;
pub mod protocols;

pub mod handler {
    use actix_web::dev::HttpServiceFactory;
    use actix_web::web;

    pub fn api_routes() -> impl HttpServiceFactory {
        web::scope("")
    }
}