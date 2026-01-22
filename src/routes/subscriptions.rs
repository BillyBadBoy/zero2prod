use actix_web::{HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Subsciption {
    name: String,
    email: String,
}

pub async fn subscribe(_: web::Form<Subsciption>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
