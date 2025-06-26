use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::config::get_conf;

#[tokio::main]
async fn main() -> std::io::Result<()> {
	let conf = get_conf().expect("Failed to read conf");
	let addr = format!("127.0.0.1:{}", conf.app_port);
	let lis = TcpListener::bind(addr).expect("main bind tcp error");
	run(lis)?.await
}
