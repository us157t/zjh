use actix_web::{
	web,
	App,
	HttpResponse,
	HttpServer
};

use actix_web::dev::Server;
async fn hc() -> HttpResponse {
	HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server,std::io::Error> {
	let s = HttpServer::new(|| {
		App::new()
			.route("/hc", web::get().to(hc))
	})
	.bind("127.0.0.1:3000")?
	.run();

	Ok(s)
}
