use sqlx::{PgConnection, Connection};
use zero2prod::config::get_conf;
use std::net::TcpListener;

#[tokio::test]
async fn test_200() {
	let app = spawn_app();
	let conf = get_conf().expect("Failed to read conf");
	let conn_string = conf.db.conn_string();
	let mut conn = PgConnection::connect(&conn_string)
		.await
		.expect("Failed to conn to Postgres");

	let cli = reqwest::Client::new();

	let body = "name=le%20guin&email=us";
	let res = cli
		.post(&format!("{}/subs", &app))
		.header("Content-Type", "application/x-www-form-urlencoded")
		.body(body)
		.send()
		.await
		.expect("Failed to exe request");

	assert_eq!(200, res.status().as_u16());
	let saved = sqlx::query!("SELECT email, name FROM subs",)
		.fetch_one(&mut conn)
		.await
		.expect("Failed to fetch saved subs");

	assert_eq!(saved.email, "1");
	assert_eq!(saved.name, "2");
}

#[tokio::test]
async fn test_400() {
	let app = spawn_app();
	let cli = reqwest::Client::new();
	let test_cases = vec![
		("name=le%20guin", "missing the email"),
		("email=us", "missing the name"),
		("", "missing both name and email")
	];

	for (body, err) in test_cases {
		let res = cli
			.post(&format!("{}/subs", &app))
			.header("Content-Type", "application/x-www-form-urlencoded")
			.body(body)
			.send()
			.await
			.expect("Failed to exe reqwest");

		assert_eq!(
			400,
			res.status().as_u16(),
			"The API did not fail with 400 when payload was {}",
			err
		);
	}

}

#[tokio::test]
async fn dum_test() {
	let app = spawn_app();
	let cli = reqwest::Client::new();
	let res = cli
		.get(format!("{}/hc",app))
		.send()
		.await
		.expect("Failed to exe request22222222222222");

	assert!(res.status().is_success());
	assert_eq!(Some(0), res.content_length());	
}

fn spawn_app()  -> String {
	let lis = TcpListener::bind("127.0.0.1:0").expect("spawn app listen error");
	let port = lis.local_addr().unwrap().port();
	let s = zero2prod::run(lis).expect("Failed to bind addr");
	let _ = tokio::spawn(s);
	format!("http://127.0.0.1:{}", port)
}
