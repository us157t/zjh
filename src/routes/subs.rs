use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;
use crate::domain::{NewSubs, SubsName};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
	name = "Adding a new subs",
	skip(_form, _conn),
	fields(
		email = %_form.email,
		name = %_form.name
	)
)]
pub async fn _subs(_form: web::Form<FormData>, _conn: web::Data<PgPool>) -> HttpResponse {
    let name = match SubsName::parse(_form.0.name) {
	Ok(name) => name,
	Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let new_subs = NewSubs {
	email: _form.0.email,
	name,
    };

    match insert_subs(&_conn, &new_subs).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "INSERT SUBS", skip(pool, form))]
pub async fn insert_subs(pool: &PgPool, form: &NewSubs) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
		INSERT INTO subs (id, email, name, subs_at)
		VALUES ($1, $2, $3, $4)
		"#,
        Uuid::new_v4(),
        form.email,
        form.name.as_ref(),
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
