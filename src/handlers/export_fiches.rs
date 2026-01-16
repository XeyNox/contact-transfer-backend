//! Handler pour l'export des fiches contacts.

use actix_web::{web, HttpRequest, HttpResponse};
use std::sync::Arc;
use tracing::{info, error, instrument};
use validator::Validate;

use crate::config::AppConfig;
use crate::domain::{
    Email, EmailAttachment, ExportFichesRequest, ExportFichesResponse,
};
use crate::email::{EmailProvider, EmailTemplates};
use crate::middleware::verify_api_key;

/// POST /api/export-fiches
///
/// Exporte les fiches contacts par email avec photos en pièces jointes.
#[instrument(skip(req, body, config, email_provider), fields(contacts_count))]
pub async fn export_fiches(
    req: HttpRequest,
    body: web::Json<ExportFichesRequest>,
    config: web::Data<Arc<AppConfig>>,
    email_provider: web::Data<Arc<dyn EmailProvider>>,
) -> HttpResponse {
    // 1. Vérifier l'authentification
    if let Err(response) = verify_api_key(&req, &config) {
        return response;
    }

    // 2. Valider la requête
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ExportFichesResponse::error(
            format!("Validation échouée: {:?}", errors)
        ));
    }

    let contacts = &body.contacts;
    
    if contacts.is_empty() {
        return HttpResponse::BadRequest().json(ExportFichesResponse::error(
            "Aucune fiche contact à exporter"
        ));
    }

    tracing::Span::current().record("contacts_count", contacts.len());

    // 3. Déterminer le destinataire
    let recipient = body
        .recipient_email
        .clone()
        .unwrap_or_else(|| config.email.default_recipient.clone());

    // 4. Construire le sujet
    let subject = body.subject.clone().unwrap_or_else(|| {
        format!("📋 Export {} fiches contacts - SMP Moules", contacts.len())
    });

    // 5. Générer le contenu HTML
    let html_body = EmailTemplates::export_fiches_html(contacts);

    // 6. Construire les pièces jointes
    let attachments: Vec<EmailAttachment> = contacts
        .iter()
        .filter_map(|c| {
            c.photo_base64.as_ref().map(|base64| {
                EmailAttachment::jpeg(c.safe_photo_filename(), base64.clone())
            })
        })
        .collect();

    let attachment_count = attachments.len();

    // 7. Construire l'email
    let email = Email {
        to: recipient.clone(),
        subject,
        html_body,
        attachments,
    };

    // 8. Envoyer via le provider
    match email_provider.send(&email).await {
        Ok(email_id) => {
            info!(
                email_id = %email_id,
                to = %recipient,
                contacts = contacts.len(),
                attachments = attachment_count,
                "Export envoyé avec succès"
            );

            HttpResponse::Ok().json(ExportFichesResponse::success(contacts.len(), &recipient))
        }
        Err(e) => {
            error!(error = %e, "Erreur envoi export");

            HttpResponse::InternalServerError().json(ExportFichesResponse::error(
                format!("Erreur d'envoi: {}", e)
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ContactFiche;
    use crate::email::provider::mock::MockEmailProvider;

    fn test_contact() -> ContactFiche {
        ContactFiche {
            societe: "Test Corp".to_string(),
            contact: "John Doe".to_string(),
            email: "john@test.com".to_string(),
            telephone: "0123456789".to_string(),
            notes: "Test notes".to_string(),
            sectors: "PHARMA".to_string(),
            status: None,
            created_at: 1704067200000,
            photo_base64: None,
            photo_filename: None,
        }
    }

    #[test]
    fn test_email_construction() {
        let contacts = vec![test_contact()];
        let html = EmailTemplates::export_fiches_html(&contacts);
        
        assert!(html.contains("Test Corp"));
        assert!(html.contains("john@test.com"));
    }
}
