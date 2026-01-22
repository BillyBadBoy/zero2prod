use actix_web::HttpResponse;
use chrono::Utc;

pub async fn date() -> HttpResponse {
    let now = Utc::now().to_rfc3339();
    HttpResponse::Ok().body(now)
}
