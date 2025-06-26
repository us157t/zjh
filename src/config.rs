#[derive(serde::Deserialize)]
pub struct Settings {
	pub db: DbSettings,
	pub app_port: u16
}

#[derive(serde::Deserialize)]
pub struct DbSettings {
	pub username: String,
	pub password: String,
	pub port: u16,
	pub host: String,
	pub db_name: String,
}

impl DbSettings {
	pub fn conn_string(&self) -> String {
		format!(
			"postgres://{}:{}@{}:{}/{}",
			self.username,
			self.password,
			self.host,
			self.port,
			self.db_name
		)
	}
}


pub fn get_conf() -> Result<Settings, config::ConfigError> {
	let mut s = config::Config::default();
	s.merge(config::File::with_name("configuration"))?;
	s.try_into()
}
