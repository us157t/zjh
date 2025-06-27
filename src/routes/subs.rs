use chrono::Utc;
use uuid::Uuid;
use sqlx::PgPool;
use actix_web::{web,HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
	email: String,
	name: String,
}

pub async fn _subs(_form: web::Form<FormData>,
		   _conn: web::Data<PgPool>,
) -> HttpResponse{
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
	.await {
		Ok(_) => HttpResponse::Ok().finish(),
		Err(e) => {
			println!("Failed to exe query: {}", e);
			HttpResponse::InternalServerError().finish()
		}
	}
} 


