//! Templates HTML pour les emails.
//!
//! Génère le contenu HTML des emails de manière isolée
//! et testable.

use crate::domain::{ContactData, ContactFiche, ContactStatus};
use chrono::Utc;

/// Générateur de templates email
pub struct EmailTemplates;

impl EmailTemplates {
    /// Génère l'email HTML pour l'export de fiches contacts
    pub fn export_fiches_html(contacts: &[ContactFiche]) -> String {
        let rows = Self::build_contact_rows(contacts);
        let now = Utc::now().format("%d/%m/%Y à %H:%M").to_string();
        let photo_count = contacts.iter().filter(|c| c.has_photo()).count();

        format!(
            r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Export Fiches Contacts - SMP Moules</title>
</head>
<body style="font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;margin:0;padding:0;background:#f5f5f5;">
    <div style="max-width:900px;margin:0 auto;background:white;">
        <!-- Header -->
        <div style="background:linear-gradient(135deg, #CC0033 0%, #990026 100%);padding:30px;text-align:center;">
            <h1 style="color:white;margin:0;font-size:28px;">📋 Export Fiches Contacts</h1>
            <p style="color:rgba(255,255,255,0.9);margin:10px 0 0 0;">SMP Moules - Application Salon</p>
        </div>

        <!-- Résumé -->
        <div style="padding:20px 30px;background:#f8f9fa;border-bottom:1px solid #eee;">
            <table style="width:100%;">
                <tr>
                    <td style="color:#333;">
                        <strong>{count}</strong> fiche(s) contact exportée(s) le <strong>{date}</strong>
                    </td>
                    <td style="text-align:right;color:#666;">
                        {photo_info}
                    </td>
                </tr>
            </table>
        </div>

        <!-- Tableau des contacts -->
        <div style="padding:20px;overflow-x:auto;">
            <table style="width:100%;border-collapse:collapse;font-size:14px;">
                <thead>
                    <tr style="background:#CC0033;color:white;">
                        <th style="padding:12px;text-align:left;border-radius:4px 0 0 0;">Société</th>
                        <th style="padding:12px;text-align:left;">Contact</th>
                        <th style="padding:12px;text-align:left;">Email</th>
                        <th style="padding:12px;text-align:left;">Téléphone</th>
                        <th style="padding:12px;text-align:left;">Secteurs</th>
                        <th style="padding:12px;text-align:left;">Notes</th>
                        <th style="padding:12px;text-align:left;border-radius:0 4px 0 0;">Statut</th>
                    </tr>
                </thead>
                <tbody>
                    {rows}
                </tbody>
            </table>
        </div>

        <!-- Footer -->
        <div style="padding:20px 30px;background:#333;color:white;text-align:center;border-radius:0 0 4px 4px;">
            {photo_footer}
            <p style="margin:10px 0 0 0;font-size:12px;color:#999;">
                SMP Moules - Expert en conception et fabrication de moules de haute précision
            </p>
        </div>
    </div>
</body>
</html>"#,
            count = contacts.len(),
            date = now,
            photo_info = if photo_count > 0 {
                format!("📷 {} photo(s) jointe(s)", photo_count)
            } else {
                String::new()
            },
            rows = rows,
            photo_footer = if photo_count > 0 {
                r#"<p style="margin:0;font-size:14px;">📷 Les photos de cartes de visite sont disponibles en pièces jointes</p>"#
            } else {
                ""
            }
        )
    }

    /// Génère l'email HTML pour l'historique des contacts
    pub fn history_email_html(contacts: &[ContactData], export_date: &str) -> String {
        let rows = Self::build_history_rows(contacts);

        format!(
            r#"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Historique Contacts - SMP Moules</title>
</head>
<body style="font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;margin:0;padding:0;background:#f5f5f5;">
    <div style="max-width:800px;margin:0 auto;background:white;">
        <!-- Header -->
        <div style="background:linear-gradient(135deg, #CC0033 0%, #990026 100%);padding:30px;text-align:center;">
            <h1 style="color:white;margin:0;font-size:24px;">📊 Historique des Contacts</h1>
            <p style="color:rgba(255,255,255,0.9);margin:10px 0 0 0;">SMP Moules</p>
        </div>

        <!-- Info -->
        <div style="padding:15px 30px;background:#f8f9fa;border-bottom:1px solid #eee;">
            <p style="margin:0;color:#333;">
                Export du <strong>{date}</strong> - <strong>{count}</strong> contact(s)
            </p>
        </div>

        <!-- Liste -->
        <div style="padding:20px;">
            {rows}
        </div>

        <!-- Footer -->
        <div style="padding:20px;background:#333;color:white;text-align:center;">
            <p style="margin:0;font-size:12px;color:#999;">
                SMP Moules - www.smp-moules.com
            </p>
        </div>
    </div>
</body>
</html>"#,
            date = export_date,
            count = contacts.len(),
            rows = rows
        )
    }

    /// Construit les lignes du tableau pour l'export
    fn build_contact_rows(contacts: &[ContactFiche]) -> String {
        contacts
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let bg = if i % 2 == 0 { "#ffffff" } else { "#f9f9f9" };
                let status_badge = Self::status_badge(&c.status);
                let photo_icon = if c.has_photo() { " 📷" } else { "" };

                format!(
                    r#"<tr style="background:{bg};border-bottom:1px solid #eee;">
                        <td style="padding:12px;font-weight:bold;">{societe}</td>
                        <td style="padding:12px;">{contact}</td>
                        <td style="padding:12px;"><a href="mailto:{email}" style="color:#CC0033;text-decoration:none;">{email}</a></td>
                        <td style="padding:12px;">{telephone}</td>
                        <td style="padding:12px;"><span style="background:#f0f0f0;padding:2px 8px;border-radius:4px;font-size:12px;">{sectors}</span></td>
                        <td style="padding:12px;font-size:12px;color:#666;max-width:150px;overflow:hidden;text-overflow:ellipsis;">{notes}</td>
                        <td style="padding:12px;">{status}{photo}</td>
                    </tr>"#,
                    bg = bg,
                    societe = html_escape(&c.societe),
                    contact = html_escape(&c.contact),
                    email = html_escape(&c.email),
                    telephone = html_escape(&c.telephone),
                    sectors = html_escape(&c.sectors),
                    notes = html_escape(&c.notes),
                    status = status_badge,
                    photo = photo_icon
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Construit les lignes pour l'historique
    fn build_history_rows(contacts: &[ContactData]) -> String {
        contacts
            .iter()
            .map(|c| {
                format!(
                    r#"<div style="border:1px solid #eee;border-radius:8px;padding:15px;margin-bottom:10px;">
                        <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:10px;">
                            <strong style="font-size:16px;color:#333;">{societe}</strong>
                            <span style="font-size:12px;color:#999;">{date}</span>
                        </div>
                        <p style="margin:5px 0;color:#666;">👤 {contact}</p>
                        <p style="margin:5px 0;"><a href="mailto:{email}" style="color:#CC0033;">{email}</a></p>
                        <p style="margin:5px 0;color:#666;">📞 {telephone}</p>
                        {notes}
                    </div>"#,
                    societe = html_escape(&c.societe),
                    contact = html_escape(&c.contact),
                    email = html_escape(&c.email),
                    telephone = html_escape(&c.telephone),
                    date = html_escape(&c.created_at),
                    notes = if c.notes.is_empty() {
                        String::new()
                    } else {
                        format!(r#"<p style="margin:10px 0 0 0;padding:10px;background:#f9f9f9;border-radius:4px;font-size:13px;color:#666;">📝 {}</p>"#, html_escape(&c.notes))
                    }
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Génère le badge de statut HTML
    fn status_badge(status: &Option<ContactStatus>) -> String {
        match status {
            Some(ContactStatus::Sent) => {
                r#"<span style="background:#4CAF50;color:white;padding:2px 8px;border-radius:12px;font-size:11px;">Envoyé</span>"#.to_string()
            }
            Some(ContactStatus::Pending) => {
                r#"<span style="background:#FF9800;color:white;padding:2px 8px;border-radius:12px;font-size:11px;">En attente</span>"#.to_string()
            }
            Some(ContactStatus::Error) => {
                r#"<span style="background:#f44336;color:white;padding:2px 8px;border-radius:12px;font-size:11px;">Erreur</span>"#.to_string()
            }
            None => String::new(),
        }
    }
}

/// Échappe les caractères HTML dangereux
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_escape("a & b"), "a &amp; b");
    }

    #[test]
    fn test_export_html_not_empty() {
        let contacts = vec![ContactFiche {
            societe: "Test".to_string(),
            contact: "John".to_string(),
            email: "john@test.com".to_string(),
            telephone: "0123456789".to_string(),
            notes: "Notes".to_string(),
            sectors: "PHARMA".to_string(),
            status: Some(ContactStatus::Sent),
            created_at: 1704067200000,
            photo_base64: None,
            photo_filename: None,
        }];

        let html = EmailTemplates::export_fiches_html(&contacts);
        assert!(html.contains("Test"));
        assert!(html.contains("john@test.com"));
        assert!(html.contains("1 fiche(s)"));
    }

    #[test]
    fn test_status_badges() {
        assert!(EmailTemplates::status_badge(&Some(ContactStatus::Sent)).contains("Envoyé"));
        assert!(EmailTemplates::status_badge(&Some(ContactStatus::Pending)).contains("En attente"));
        assert!(EmailTemplates::status_badge(&None).is_empty());
    }
}
