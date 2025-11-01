use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}


pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    log::info!("Request id: {} - adding {}, {} as new subscriber!", request_id, form.email, form.name);
    log::info!("Request id: {} - saving new subscriber to the db!", request_id);
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at_timestamp)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("Request id: {} - new subscriber was saved!", request_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::info!("Request id: {} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
    
}
