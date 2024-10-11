use actix_web::{post, web, HttpResponse, Responder};

#[post("/transform")]
async fn transform_message(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("Transformed message")
}

#[post("/validate")]
async fn validate_message(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("Validated message")
}
