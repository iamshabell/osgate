use actix_web::{post, HttpResponse, Responder};

use crate::message::{transformer::Transformer, xml_validator};

#[post("/transform")]
async fn transform_message(req_body: String) -> impl Responder {
    let mut transformer = Transformer::new(&req_body);

    match transformer.xml_to_json() {
        Ok(json) => HttpResponse::Ok().json(json),
        Err(_) => HttpResponse::BadRequest().body("Failed to transform message."),
    }
}

#[post("/transform/json")]
async fn transform_json_message(req_body: String) -> impl Responder {
    let mut transformer = Transformer::new(&req_body);

    match transformer.json_to_xml() {
        Ok(xml) => HttpResponse::Ok().body(xml),
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
