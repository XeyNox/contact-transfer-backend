//! Implémentation du provider Resend.

use super::{EmailError, EmailProvider, EmailResult};
use crate::config::EmailConfig;
use crate::domain::Email;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

const RESEND_API_URL: &str = "https://api.resend.com/emails";

/// Provider Resend pour l'envoi d'emails
pub struct ResendProvider {
    client: Client,
    api_key: String,
    from_address: String,
}

impl ResendProvider {
    /// Crée un nouveau provider Resend
    pub fn new(config: &EmailConfig) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            api_key: config.resend_api_key.clone(),
            from_address: format!("{} <{}>", config.from_name, config.from_email),
        }
    }
}

#[async_trait]
impl EmailProvider for ResendProvider {
    async fn send(&self, email: &Email) -> EmailResult<String> {
        debug!(
            to = %email.to,
            subject = %email.subject,
            attachments = email.attachments.len(),
            "Envoi email via Resend"
        );

        // Construire la requête Resend
        let attachments: Vec<ResendAttachment> = email
            .attachments
            .iter()
            .map(|a| ResendAttachment {
                filename: a.filename.clone(),
                content: a.content_base64.clone(),
            })
            .collect();

        let request = ResendEmailRequest {
            from: self.from_address.clone(),
            to: vec![email.to.clone()],
            subject: email.subject.clone(),
            html: email.html_body.clone(),
            attachments: if attachments.is_empty() {
                None
            } else {
                Some(attachments)
            },
        };

        // Envoyer la requête
        let response = self
            .client
            .post(RESEND_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!(error = %e, "Erreur connexion Resend");
                EmailError::ConnectionError(e.to_string())
            })?;

        let status = response.status();

        if status.is_success() {
            let result: ResendSuccessResponse = response.json().await.map_err(|e| {
                EmailError::ProviderError(format!("Erreur parsing réponse: {}", e))
            })?;

            info!(email_id = %result.id, to = %email.to, "Email envoyé avec succès");
            Ok(result.id)
        } else {
            let error_body = response.text().await.unwrap_or_default();

            // Classifier l'erreur selon le code HTTP
            let error = match status.as_u16() {
                401 => EmailError::AuthenticationError("Clé API invalide".to_string()),
                422 => EmailError::InvalidRequest(error_body),
                429 => EmailError::RateLimited,
                _ => EmailError::ProviderError(format!("HTTP {}: {}", status, error_body)),
            };

            error!(
                status = %status,
                error = %error,
                "Erreur Resend"
            );

            Err(error)
        }
    }

    async fn health_check(&self) -> bool {
        // Simple check: on peut créer une connexion
        self.client
            .get("https://api.resend.com")
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
            .is_ok()
    }

    fn provider_name(&self) -> &'static str {
        "resend"
    }
}

// =============================================================================
// STRUCTURES RESEND API
// =============================================================================

#[derive(Debug, Serialize)]
struct ResendEmailRequest {
    from: String,
    to: Vec<String>,
    subject: String,
    html: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<ResendAttachment>>,
}

#[derive(Debug, Serialize)]
struct ResendAttachment {
    filename: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ResendSuccessResponse {
    id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::EmailConfig;

    fn test_config() -> EmailConfig {
        EmailConfig {
            resend_api_key: "test_key".to_string(),
            from_name: "Test".to_string(),
            from_email: "test@example.com".to_string(),
            default_recipient: "recipient@example.com".to_string(),
        }
    }

    #[test]
    fn test_provider_creation() {
        let config = test_config();
        let provider = ResendProvider::new(&config);
        assert_eq!(provider.provider_name(), "resend");
    }
}
