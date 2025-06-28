use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
	name = "Adding a new subs",
	skip(_form, _conn),
	fields(
		req_id = %Uuid::new_v4(),
		email = %_form.email,
		name = %_form.name
	)
)]
pub async fn _subs(_form: web::Form<FormData>, _conn: web::Data<PgPool>) -> HttpResponse {
	match insert_subs(&_conn, &_form).await {
		Ok(_) => HttpResponse::Ok().finish(),
		Err(_) => HttpResponse::InternalServerError().finish()
	}	
}

#[tracing::instrument(
	name = "INSERT SUBS",
	skip(pool, form)
)]
pub async fn insert_subs(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
		INSERT INTO subs (id, email, name, subs_at)
		VALUES ($1, $2, $3, $4)
		"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
	tracing::error!("Failed to exe query: {:?}", e);
	e
    })?;
    Ok(())
}
