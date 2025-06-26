use std::net::TcpListener;
use crate::routes::{_subs,_hc};
use actix_web::dev::Server;
use actix_web::{
	App,
	web,
	HttpServer,
};

pub fn run(lis: TcpListener) -> Result<Server,std::io::Error> {
        let s = HttpServer::new(|| {
                App::new()
                        .route("/hc", web::get().to(_hc))
                        .route("/subs", web::post().to(_subs))
        })
        .listen(lis)?
        .run();

        Ok(s) 
}
