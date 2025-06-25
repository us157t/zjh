use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
	let lis = TcpListener::bind("127.0.0.1:0").expect("main bind tcp error");
	let port = lis.local_addr().unwrap().port();
	println!("Main port {}", port);
	run(lis)?.await
}
