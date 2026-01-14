use serde::Deserialize;
use std::env;

#[derive(Clone, Debug, Deserialize)]
pub struct EmailConfig {
    // PERSONNALISER : Adapter selon votre service SMTP
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
}

impl EmailConfig {
    pub fn from_env() -> Self {
        Self {
            smtp_host: env::var("SMTP_HOST")
                .expect("SMTP_HOST doit être défini"),
            smtp_port: env::var("SMTP_PORT")
                .expect("SMTP_PORT doit être défini")
                .parse()
                .expect("SMTP_PORT doit être un nombre"),
            smtp_username: env::var("SMTP_USERNAME")
                .expect("SMTP_USERNAME doit être défini"),
            smtp_password: env::var("SMTP_PASSWORD")
                .expect("SMTP_PASSWORD doit être défini"),
            from_email: env::var("SMTP_FROM_EMAIL")
                .expect("SMTP_FROM_EMAIL doit être défini"),
        }
    }
}
