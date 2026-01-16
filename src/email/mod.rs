//! Abstraction pour l'envoi d'emails.
//!
//! Utilise le pattern trait object pour permettre:
//! - Injection de dépendances
//! - Tests avec mock
//! - Changement de provider sans modifier le code métier

mod provider;
mod resend;
mod templates;

pub use provider::{EmailProvider, EmailError, EmailResult};
pub use resend::ResendProvider;
pub use templates::EmailTemplates;
