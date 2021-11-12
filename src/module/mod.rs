pub mod user;
pub mod ws;

pub mod handler {
    use actix_web::dev::HttpServiceFactory;
    use actix_web::web;

    pub fn api_routes() -> impl HttpServiceFactory {
        web::scope("")
    }
}