use actix_web::{web,HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
	email: String,
	name: String,
}

pub async fn _subs(_form: web::Form<FormData>) -> HttpResponse{
	HttpResponse::Ok().finish()
} 


