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

pub async fn _subs(_form: web::Form<FormData>, _conn: web::Data<PgPool>) -> HttpResponse {
    let req_id = Uuid::new_v4();
    
    let span = tracing::info_span!(
	"Adding a new",
	%req_id,
	email = %_form.email,
	name = %_form.name
    );

    tracing::info!("Saving new subs now!!!");
    let _guard = span.enter();
    let query_span = tracing::info_span!(
"234Saving new subscriber details in the database"
);
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
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to exe query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
