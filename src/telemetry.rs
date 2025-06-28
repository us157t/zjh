use sqlx::Connection;
use sqlx::PgPool;
use std::net::TcpListener;
use uuid::Uuid;
//use crate::config::get_conf;
//use crate::startup::run;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{
	BunyanFormattingLayer,
	JsonStorageLayer
};
use tracing_log::LogTracer;
use tracing_subscriber::{
	layer::SubscriberExt,
	EnvFilter,
	Registry
};

use tracing::Subscriber;
pub fn init_subs(name: String, ef: String) {
    LogTracer::init().expect("Failed to set logger");
    let ef = EnvFilter::try_from_default_env()
	.unwrap_or_else(|_| EnvFilter::new(ef));

    let layer = BunyanFormattingLayer::new(
	name,
	std::io::stdout
    );
    let subs = Registry::default()
	.with(ef)
	.with(JsonStorageLayer)
	.with(layer);

     set_global_default(subs)
        .expect("Failed to set susbs");

}
