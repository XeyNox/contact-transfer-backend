//! Middleware et guards pour la sécurité.

use actix_web::{HttpRequest, HttpResponse};
use crate::config::AppConfig;
use crate::domain::ExportFichesResponse;

/// Vérifie la clé API dans les headers
pub fn verify_api_key(req: &HttpRequest, config: &AppConfig) -> Result<(), HttpResponse> {
    let api_key = req
        .headers()
        .get("X-API-Key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if api_key == config.security.api_key {
        Ok(())
    } else {
        tracing::warn!(
            remote_addr = ?req.connection_info().peer_addr(),
            "Tentative d'accès avec clé API invalide"
        );
        Err(HttpResponse::Unauthorized().json(ExportFichesResponse::error("Clé API invalide")))
    }
}
