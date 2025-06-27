use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn _subs(_form: web::Form<FormData>, _conn: web::Data<PgPool>) -> HttpResponse {
    log::info!(
        "Adding '{}' '{}' as a new subscriber.",
        _form.email,
        _form.name
    );
    log::info!("Saving new subs now!!!");
    match sqlx::query!(
        r#"
		INSERT INTO subs (id, email, name, subs_at)
		VALUES ($1, $2, $3, $4)
		"#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    )
    .execute(_conn.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("Failed to exe query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
