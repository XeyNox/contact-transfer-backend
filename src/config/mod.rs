//! Configuration centralisée de l'application.
//!
//! Charge la configuration depuis les variables d'environnement
//! et fournit un accès typé aux paramètres.

use serde::Deserialize;

/// Configuration complète de l'application
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub email: EmailConfig,
    pub security: SecurityConfig,
}

/// Configuration du serveur HTTP
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

/// Configuration email (Resend)
#[derive(Debug, Clone, Deserialize)]
pub struct EmailConfig {
    pub resend_api_key: String,
    pub from_name: String,
    pub from_email: String,
    pub default_recipient: String,
}

/// Configuration de sécurité
#[derive(Debug, Clone, Deserialize)]
pub struct SecurityConfig {
    pub api_key: String,
}

// Valeurs par défaut
fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8080
}

impl AppConfig {
    /// Charge la configuration depuis les variables d'environnement
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        Ok(Self {
            server: ServerConfig {
                host: std::env::var("HOST").unwrap_or_else(|_| default_host()),
                port: std::env::var("PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or_else(default_port),
            },
            email: EmailConfig {
                resend_api_key: std::env::var("RESEND_API_KEY")
                    .map_err(|_| ConfigError::MissingEnvVar("RESEND_API_KEY"))?,
                from_name: std::env::var("EMAIL_FROM_NAME")
                    .unwrap_or_else(|_| "SMP Moules".to_string()),
                from_email: std::env::var("EMAIL_FROM_ADDRESS")
                    .unwrap_or_else(|_| "noreply@smp-moules.com".to_string()),
                default_recipient: std::env::var("DEFAULT_EXPORT_EMAIL")
                    .unwrap_or_else(|_| "commercial@smp-moules.com".to_string()),
            },
            security: SecurityConfig {
                api_key: std::env::var("API_KEY")
                    .unwrap_or_else(|_| "dev-api-key".to_string()),
            },
        })
    }
}

/// Erreurs de configuration
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Variable d'environnement manquante: {0}")]
    MissingEnvVar(&'static str),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        assert_eq!(default_host(), "0.0.0.0");
        assert_eq!(default_port(), 8080);
    }
}
