use crate::domain::SubsEmail;
use secrecy::ExposeSecret;
use secrecy::Secret;

#[derive(serde::Deserialize)]
pub struct EmailCliSettings {
    pub base_url: String,
    pub sender_email: String,
}

impl EmailCliSettings {
    pub fn sender(&self) -> Result<SubsEmail, String> {
        SubsEmail::parse(self.sender_email.clone())
    }
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub db: DbSettings,
    pub app_port: u16,
    pub email_cli: EmailCliSettings,
}

#[derive(serde::Deserialize)]
pub struct DbSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub db_name: String,
}

impl DbSettings {
    pub fn conn_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.db_name
        ))
    }

    pub fn conn_string2(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
        ))
    }
}

pub fn get_conf() -> Result<Settings, config::ConfigError> {
    let mut s = config::Config::default();
    s.merge(config::File::with_name("configuration"))?;
    s.try_into()
}
