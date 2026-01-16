//! Types de domaine pour l'application.
//!
//! Ces structures représentent les entités métier indépendamment
//! de la couche HTTP ou du provider email.

use serde::{Deserialize, Serialize};
use validator::Validate;

// =============================================================================
// CONTACT
// =============================================================================

/// Une fiche contact collectée lors d'un salon
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ContactFiche {
    #[validate(length(min = 1, message = "Société requise"))]
    pub societe: String,

    pub contact: String,

    #[validate(email(message = "Email invalide"))]
    pub email: String,

    pub telephone: String,

    pub notes: String,

    pub sectors: String,

    #[serde(default)]
    pub status: Option<ContactStatus>,

    pub created_at: i64,

    /// Photo de carte de visite en base64
    #[serde(default)]
    pub photo_base64: Option<String>,

    /// Nom du fichier photo
    #[serde(default)]
    pub photo_filename: Option<String>,
}

/// Statut d'un contact
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ContactStatus {
    Pending,
    Sent,
    Error,
}

impl Default for ContactStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl ContactFiche {
    /// Vérifie si le contact a une photo
    pub fn has_photo(&self) -> bool {
        self.photo_base64.is_some()
    }

    /// Génère un nom de fichier sécurisé pour la photo
    pub fn safe_photo_filename(&self) -> String {
        self.photo_filename
            .clone()
            .unwrap_or_else(|| {
                let safe_name: String = self.societe
                    .chars()
                    .map(|c| if c.is_alphanumeric() { c } else { '_' })
                    .collect();
                format!("carte_visite_{}.jpg", safe_name)
            })
    }
}

// =============================================================================
// EMAIL
// =============================================================================

/// Un email à envoyer
#[derive(Debug, Clone)]
pub struct Email {
    pub to: String,
    pub subject: String,
    pub html_body: String,
    pub attachments: Vec<EmailAttachment>,
}

/// Pièce jointe d'un email
#[derive(Debug, Clone)]
pub struct EmailAttachment {
    pub filename: String,
    pub content_base64: String,
    #[allow(dead_code)]
    pub content_type: String,
}

impl EmailAttachment {
    pub fn jpeg(filename: String, content_base64: String) -> Self {
        Self {
            filename,
            content_base64,
            content_type: "image/jpeg".to_string(),
        }
    }
}

// =============================================================================
// EXPORT REQUEST/RESPONSE
// =============================================================================

/// Requête d'export de fiches
#[derive(Debug, Deserialize, Validate)]
pub struct ExportFichesRequest {
    #[validate(length(min = 1, message = "Au moins un contact requis"))]
    pub contacts: Vec<ContactFiche>,

    #[validate(email(message = "Email destinataire invalide"))]
    pub recipient_email: Option<String>,

    pub subject: Option<String>,

    #[allow(dead_code)]
    pub export_date: Option<i64>,

    #[allow(dead_code)]
    pub app_version: Option<String>,
}

/// Réponse d'export
#[derive(Debug, Serialize)]
pub struct ExportFichesResponse {
    pub success: bool,
    pub message: String,
    pub contacts_count: usize,
}

impl ExportFichesResponse {
    pub fn success(count: usize, recipient: &str) -> Self {
        Self {
            success: true,
            message: format!("{} fiche(s) envoyée(s) avec succès à {}", count, recipient),
            contacts_count: count,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            contacts_count: 0,
        }
    }
}

// =============================================================================
// HISTORY EMAIL REQUEST
// =============================================================================

/// Requête d'envoi d'historique
#[derive(Debug, Deserialize, Validate)]
pub struct HistoryEmailRequest {
    #[validate(email)]
    pub recipient_email: String,

    #[validate(length(min = 1))]
    pub contacts: Vec<ContactData>,

    #[allow(dead_code)]
    pub total_contacts: usize,

    pub export_date: String,
}

/// Données de contact simplifiées pour l'historique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactData {
    pub societe: String,
    pub contact: String,
    pub email: String,
    pub telephone: String,
    pub notes: String,
    pub sectors: String,
    pub status: String,
    pub created_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_photo_filename() {
        let contact = ContactFiche {
            societe: "Test Company & Co.".to_string(),
            contact: "".to_string(),
            email: "".to_string(),
            telephone: "".to_string(),
            notes: "".to_string(),
            sectors: "".to_string(),
            status: None,
            created_at: 0,
            photo_base64: None,
            photo_filename: None,
        };

        let filename = contact.safe_photo_filename();
        assert!(filename.starts_with("carte_visite_"));
        assert!(!filename.contains('&'));
        assert!(!filename.contains(' '));
    }
}
