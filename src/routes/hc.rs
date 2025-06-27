use actix_web::{web, HttpResponse};
pub async fn _hc() -> HttpResponse {
    HttpResponse::Ok().finish()
}
