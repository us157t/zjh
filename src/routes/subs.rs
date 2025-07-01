use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;
use crate::domain::{NewSubs, SubsName, SubsEmail};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}


impl TryFrom<FormData> for NewSubs {
	type Error = String;
	
	fn try_from(v: FormData) -> Result<Self, Self::Error> {
		let name = SubsName::parse(v.name)?;
		let email = SubsEmail::parse(v.email)?;
		Ok(Self { email, name })
	}
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
    let new_subs = match _form.0.try_into() {
	Ok(s) => s,
	Err(_) => return HttpResponse::BadRequest().finish(),
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
        form.email.as_ref(),
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
