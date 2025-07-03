use crate::email_client::EmailCli;
use crate::routes::{_hc, _subs};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(lis: TcpListener, conn: PgPool, email_cli: EmailCli) -> Result<Server, std::io::Error> {
    let conn = web::Data::new(conn);
    let email_cli = web::Data::new(email_cli);
    let s = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/hc", web::get().to(_hc))
            .route("/subs", web::post().to(_subs))
            .app_data(conn.clone())
            .app_data(email_cli.clone())
    })
    .listen(lis)?
    .run();

    Ok(s)
}
