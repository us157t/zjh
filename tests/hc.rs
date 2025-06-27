use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::config::get_conf;
use zero2prod::config::DbSettings;
use zero2prod::startup::run;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[tokio::test]
async fn test_200() {
    let app = spawn_app().await;
    let conf = get_conf().expect("Failed to read conf");
    let conn_string = conf.db.conn_string();
    let mut conn = PgConnection::connect(&conn_string)
        .await
        .expect("Failed to conn to Postgres");

    let cli = reqwest::Client::new();

    let body = "name=le%20guin&email=us";
    let res = cli
        .post(&format!("{}/subs", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to exe request");

    assert_eq!(200, res.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subs",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subs");

    assert_eq!(saved.email, "us");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn test_400() {
    let app = spawn_app().await;
    let cli = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=us", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (body, err) in test_cases {
        let res = cli
            .post(&format!("{}/subs", &app.address))
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
    let app = spawn_app().await;
    let cli = reqwest::Client::new();
    let res = cli
        .get(format!("{}/hc", app.address))
        .send()
        .await
        .expect("Failed to exe request22222222222222");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

pub async fn conf_db(conf: &DbSettings) -> PgPool {
    let mut conn = PgPool::connect(&conf.conn_string2())
        .await
        .expect("Failed to conn to postgres");
    conn.execute(
        format!(
            r#"CREATE DATABASE "{}";
		"#,
            conf.db_name
        )
        .as_str(),
    )
    .await
    .expect("Failed to create db");

    let conn_pool = PgPool::connect(&conf.conn_string())
        .await
        .expect("Failed to conn a postgres");

    sqlx::migrate!("./migrations")
        .run(&conn_pool)
        .await
        .expect("Failed to migrate the db");

    conn_pool
}

async fn spawn_app() -> TestApp {
    let lis = TcpListener::bind("127.0.0.1:0").expect("spawn app listen error");
    let port = lis.local_addr().unwrap().port();
    let mut conf = get_conf().expect("Failed to read conf");
    conf.db.db_name = Uuid::new_v4().to_string();
    let db_pool = conf_db(&conf.db).await;
    let s = run(lis, db_pool.clone()).expect("Failed to bind addr");
    let _ = tokio::spawn(s);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool,
    }
}
