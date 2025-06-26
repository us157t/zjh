use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::config::get_conf;
use sqlx::PgPool;
use uuid::Uuid;

#[tokio::main]
async fn main() -> std::io::Result<()> {
	let mut conf = get_conf().expect("Failed to read conf");
	conf.db.db_name = Uuid::new_v4().to_string();
	let conn = PgPool::connect(&conf.db.conn_string())
		.await
		.expect("Failed to conn postgres");
	let addr = format!("127.0.0.1:{}", conf.app_port);
	let lis = TcpListener::bind(addr).expect("main bind tcp error");
	run(lis, conn)?.await
}
