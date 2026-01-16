//! Trait d'abstraction pour les providers email.

use crate::domain::Email;
use async_trait::async_trait;

/// Résultat d'envoi d'email
pub type EmailResult<T> = Result<T, EmailError>;

/// Erreurs possibles lors de l'envoi d'email
#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("Erreur de connexion au service email: {0}")]
    ConnectionError(String),

    #[error("Erreur d'authentification: {0}")]
    AuthenticationError(String),

    #[error("Requête invalide: {0}")]
    InvalidRequest(String),

    #[error("Limite de taux dépassée")]
    RateLimited,

    #[allow(dead_code)]
    #[error("Destinataire invalide: {0}")]
    InvalidRecipient(String),

    #[error("Erreur interne du service: {0}")]
    ProviderError(String),
}

/// Trait pour les providers d'email
///
/// Permet d'abstraire le service d'envoi d'email concret (Resend, SendGrid, etc.)
/// et facilite les tests avec des mocks.
#[async_trait]
pub trait EmailProvider: Send + Sync {
    /// Envoie un email
    async fn send(&self, email: &Email) -> EmailResult<String>;

    /// Vérifie si le service est disponible
    async fn health_check(&self) -> bool;

    /// Retourne le nom du provider (pour les logs)
    fn provider_name(&self) -> &'static str;
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    /// Mock provider pour les tests
    pub struct MockEmailProvider {
        pub should_succeed: bool,
        pub send_count: Arc<AtomicUsize>,
    }

    impl MockEmailProvider {
        pub fn new(should_succeed: bool) -> Self {
            Self {
                should_succeed,
                send_count: Arc::new(AtomicUsize::new(0)),
            }
        }

        pub fn get_send_count(&self) -> usize {
            self.send_count.load(Ordering::SeqCst)
        }
    }

    #[async_trait]
    impl EmailProvider for MockEmailProvider {
        async fn send(&self, _email: &Email) -> EmailResult<String> {
            self.send_count.fetch_add(1, Ordering::SeqCst);
            if self.should_succeed {
                Ok("mock-email-id".to_string())
            } else {
                Err(EmailError::ProviderError("Mock error".to_string()))
            }
        }

        async fn health_check(&self) -> bool {
            self.should_succeed
        }

        fn provider_name(&self) -> &'static str {
            "mock"
        }
    }
}
