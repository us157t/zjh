pub mod config;
pub mod domain;
pub mod email_client;
pub mod routes;
pub mod startup;
pub mod telemetry;

use actix_web::{web, App, HttpResponse, HttpServer};

use actix_web::dev::Server;
use std::net::TcpListener;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

fn index(form: web::Form<FormData>) -> String {
    format!("Welcome {}!", form.name)
}

async fn hc() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subs(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(lis: TcpListener) -> Result<Server, std::io::Error> {
    let s = HttpServer::new(|| {
        App::new()
            .route("/hc", web::get().to(hc))
            .route("/subs", web::post().to(subs))
    })
    .listen(lis)?
    .run();

    Ok(s)
}
