use actix_web::middleware::Logger;
use std::net::TcpListener;
use crate::routes::{_subs,_hc};
use actix_web::dev::Server;
use actix_web::{
	App,
	web,
	HttpServer,
};
use sqlx::PgPool;

pub fn run(lis: TcpListener,
	   conn: PgPool
) -> Result<Server,std::io::Error> {
	let conn = web::Data::new(conn);
        let s = HttpServer::new(move || {
                App::new()
			.wrap(Logger::default())
                        .route("/hc", web::get().to(_hc))
                        .route("/subs", web::post().to(_subs))
			.app_data(conn.clone())
        })
        .listen(lis)?
        .run();

        Ok(s) 
}
