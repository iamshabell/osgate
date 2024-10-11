use actix_web::{post, web, HttpResponse, Responder};

use crate::message::{transformer, xml_validator};

#[post("/transform")]
async fn transform_message(req_body: String) -> impl Responder {
    match transformer::xml_to_json(&req_body) {
        Ok(json) => HttpResponse::Ok().json(json),
        Err(_) => HttpResponse::BadRequest().body("Failed to transform message."),
    }
}

#[post("/validate")]
async fn validate_message(req_body: String) -> impl Responder {
    match xml_validator::xml_validator(&req_body) {
        true => HttpResponse::Ok().body("Message is valid"),
        false => HttpResponse::BadRequest().body("Oops! Message is not invalid"),
    }
}
