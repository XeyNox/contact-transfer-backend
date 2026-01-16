//! Handler pour l'envoi d'historique par email.

use actix_web::{web, HttpRequest, HttpResponse};
use serde::Serialize;
use std::sync::Arc;
use tracing::{info, error, instrument};
use validator::Validate;

use crate::config::AppConfig;
use crate::domain::{Email, HistoryEmailRequest};
use crate::email::{EmailProvider, EmailTemplates};
use crate::middleware::verify_api_key;

#[derive(Serialize)]
pub struct HistoryEmailResponse {
    success: bool,
    message: String,
    contacts_sent: usize,
}

impl HistoryEmailResponse {
    fn success(count: usize) -> Self {
        Self {
            success: true,
            message: format!("Historique de {} contact(s) envoyé avec succès", count),
            contacts_sent: count,
        }
    }

    fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            contacts_sent: 0,
        }
    }
}

/// POST /api/send-history-email
///
/// Envoie l'historique des contacts par email.
#[instrument(skip(req, body, config, email_provider), fields(contacts_count, recipient))]
pub async fn send_history_email(
    req: HttpRequest,
    body: web::Json<HistoryEmailRequest>,
    config: web::Data<Arc<AppConfig>>,
    email_provider: web::Data<Arc<dyn EmailProvider>>,
) -> HttpResponse {
    // 1. Vérifier l'authentification
    if let Err(response) = verify_api_key(&req, &config) {
        return response;
    }

    // 2. Valider la requête
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(HistoryEmailResponse::error(
            format!("Validation échouée: {:?}", errors)
        ));
    }

    let contacts = &body.contacts;
    let recipient = &body.recipient_email;

    if contacts.is_empty() {
        return HttpResponse::BadRequest().json(HistoryEmailResponse::error(
            "Aucun contact dans l'historique"
        ));
    }

    tracing::Span::current().record("contacts_count", contacts.len());
    tracing::Span::current().record("recipient", recipient.as_str());

    // 3. Construire le sujet
    let subject = format!(
        "📊 Historique {} contacts - SMP Moules ({})",
        contacts.len(),
        body.export_date
    );

    // 4. Générer le HTML
    let html_body = EmailTemplates::history_email_html(contacts, &body.export_date);

    // 5. Construire et envoyer l'email
    let email = Email {
        to: recipient.clone(),
        subject,
        html_body,
        attachments: vec![], // Pas de pièces jointes pour l'historique
    };

    match email_provider.send(&email).await {
        Ok(email_id) => {
            info!(
                email_id = %email_id,
                to = %recipient,
                contacts = contacts.len(),
                "Historique envoyé avec succès"
            );

            HttpResponse::Ok().json(HistoryEmailResponse::success(contacts.len()))
        }
        Err(e) => {
            error!(error = %e, to = %recipient, "Erreur envoi historique");

            HttpResponse::InternalServerError().json(HistoryEmailResponse::error(
                format!("Erreur d'envoi: {}", e)
            ))
        }
    }
}
