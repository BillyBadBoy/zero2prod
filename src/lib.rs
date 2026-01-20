use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};
use chrono::Utc;
use serde::Deserialize;
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn date() -> HttpResponse {
    let now = Utc::now().to_rfc3339();
    HttpResponse::Ok().body(now)
}

#[derive(Deserialize)]
struct Subsciption {
    name: String,
    email: String,
}

async fn subscriptions(_: web::Form<Subsciption>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/date", web::get().to(date))
            .route("/subscriptions", web::post().to(subscriptions))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
