use crate::config::EmailConfig;
use crate::errors::AppError;
use crate::models::Contact;
use lettre::smtp::authentication::Credentials;
use lettre::smtp::SmtpClient;
use lettre::Transport;
use lettre::Message;
use std::str::FromStr;

pub struct EmailService {
    config: EmailConfig,
}

impl EmailService {
    pub fn new(config: EmailConfig) -> Self {
        Self { config }
    }

    // PERSONNALISER : Générer le corps du template HTML selon vos besoins
    fn generate_html_template(&self, contacts: &[Contact]) -> String {
        let mut html = String::from(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        body { font-family: Arial, sans-serif; background-color: #f5f5f5; }
        .container { max-width: 600px; margin: 20px auto; background: white; padding: 20px; border-radius: 8px; }
        h1 { color: #333; border-bottom: 2px solid #007bff; padding-bottom: 10px; }
        .contact-item { margin: 15px 0; padding: 15px; background: #f9f9f9; border-left: 4px solid #007bff; }
        .contact-item h3 { margin: 0 0 10px 0; color: #007bff; }
        .contact-field { margin: 5px 0; }
        .label { font-weight: bold; color: #666; }
        .footer { margin-top: 30px; padding-top: 15px; border-top: 1px solid #ddd; color: #999; font-size: 12px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>📋 Export de Contacts</h1>"#
        );

        // PERSONNALISER : Ajouter/supprimer les champs affichés
        for contact in contacts {
            html.push_str(&format!(
                r#"
        <div class="contact-item">
            <h3>{} {}</h3>
            <div class="contact-field"><span class="label">Email:</span> {}</div>"#,
                contact.first_name, contact.last_name, contact.email
            ));

            if let Some(phone) = &contact.phone {
                html.push_str(&format!(
                    "<div class=\"contact-field\"><span class=\"label\">Téléphone:</span> {}</div>",
                    phone
                ));
            }

            if let Some(company) = &contact.company {
                html.push_str(&format!(
                    "<div class=\"contact-field\"><span class=\"label\">Entreprise:</span> {}</div>",
                    company
                ));
            }

            if let Some(sector) = &contact.sector {
                html.push_str(&format!(
                    "<div class=\"contact-field\"><span class=\"label\">Secteur:</span> {}</div>",
                    sector
                ));
            }

            html.push_str(&format!(
                "<div class=\"contact-field\"><span class=\"label\">Statut:</span> {}</div>",
                contact.status
            ));

            html.push_str("</div>");
        }

        html.push_str(
            r#"
        <div class="footer">
            <p>Cet email a été généré automatiquement par le système de gestion des contacts.</p>
            <p>© 2026 Contact Transfer System. Tous droits réservés.</p>
        </div>
    </div>
</body>
</html>"#
        );

        html
    }

    // PERSONNALISER : Adapter le sujet et le format selon vos besoins
    pub async fn send_contacts(&self, recipient: &str, contacts: &[Contact]) -> Result<String, AppError> {
        log::info!("Préparation de l'envoi d'email à {} avec {} contacts", recipient, contacts.len());

        let html_body = self.generate_html_template(contacts);

        let email = Message::builder()
            .from(self.config.from_email.parse()
                .map_err(|_| AppError::EmailError("Adresse email invalide".to_string()))?)
            .to(recipient.parse()
                .map_err(|_| AppError::EmailError("Destinataire email invalide".to_string()))?)
            .subject(format!("📋 Historique de contacts ({})", contacts.len()))
            .multipart(
                lettre::message::MultiPart::alternative()
                    .singlepart(lettre::message::SinglePart::plain(
                        format!("Vous avez reçu {} contacts en pièce jointe.", contacts.len())
                    ))
                    .singlepart(lettre::message::SinglePart::html(html_body))
            )
            .map_err(|e| AppError::EmailError(format!("Erreur construction email: {}", e)))?;

        // PERSONNALISER : Adapter selon votre service SMTP
        let mut client = SmtpClient::new_simple(&self.config.smtp_host)
            .map_err(|e| AppError::EmailError(format!("Erreur SMTP: {}", e)))?
            .credentials(Credentials::new(
                self.config.smtp_username.clone(),
                self.config.smtp_password.clone(),
            ))
            .transport();

        client.send(email.try_into()
            .map_err(|e| AppError::EmailError(format!("Erreur sérialisation: {}", e)))?)
            .map_err(|e| AppError::EmailError(format!("Erreur envoi: {}", e)))?;

        log::info!("Email envoyé avec succès à {}", recipient);
        Ok(format!("Email envoyé avec succès à {}", recipient))
    }
}
