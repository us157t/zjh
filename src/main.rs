use secrecy::ExposeSecret;
use sqlx::Connection;
use sqlx::PgPool;
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::config::get_conf;
use zero2prod::startup::run;
use zero2prod::telemetry::init_subs;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_subs("zero2prod".into(), "info".into(), std::io::stdout);
    let mut conf = get_conf().expect("Failed to read conf");
    //	conf.db.db_name = Uuid::new_v4().to_string();
    let conn = PgPool::connect(&conf.db.conn_string().expose_secret())
        .await
        .expect("Failed to conn postgres");
    tracing::info!("=======!!!!!");
    tracing::info!("SSSSSSSSSSSSS {:?}", &conf.db.conn_string().expose_secret());
    let addr = format!("127.0.0.1:{}", conf.app_port);
    let lis = TcpListener::bind(addr).expect("main bind tcp error");
    run(lis, conn)?.await
}
