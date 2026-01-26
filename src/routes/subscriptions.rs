use actix_web::{HttpResponse, web};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use sqlx::types::Uuid;

#[derive(Deserialize)]
pub struct Subscription {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<Subscription>, db_pool: web::Data<PgPool>) -> HttpResponse {
    // generate random request ID
    let request_id = Uuid::new_v4();

    log::info!(
        "req {}: Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name
    );

    log::info!(
        "req {}: Saving new subscriber details in the database",
        request_id
    );
    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            log::info!("req {}: New subscriber details have been saved", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("req {}: Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
