use crate::routes::{_hc, _subs};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(lis: TcpListener, conn: PgPool) -> Result<Server, std::io::Error> {
    let conn = web::Data::new(conn);
    let s = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/hc", web::get().to(_hc))
            .route("/subs", web::post().to(_subs))
            .app_data(conn.clone())
    })
    .listen(lis)?
    .run();

    Ok(s)
}
