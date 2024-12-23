use actix_web::{web, App, HttpServer};

mod api;
mod lib;
mod message;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::transform_message)
            .service(api::transform_json_message)
            .service(api::validate_message)
            .route("/hey", web::get().to(|| async { "Hello, world!" }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
