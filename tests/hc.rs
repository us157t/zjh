
#[tokio::test]
async fn dum_test() {
	spawn_app();
	let cli = reqwest::Client::new();
	let res = cli
		.get("http://127.0.0.1:3000/hc")
		.send()
		.await
		.expect("Failed to exe request22222222222222");

	assert!(res.status().is_success());
	assert_eq!(Some(0), res.content_length());	
}

fn spawn_app()  {
	let s = zero2prod::run().expect("Failed to bind addr");
	let _ = tokio::spawn(s);
	
}
