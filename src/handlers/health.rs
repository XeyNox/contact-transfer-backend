//! Handler pour le health check.

use actix_web::{web, HttpResponse};
use serde::Serialize;
use std::sync::Arc;

use crate::email::EmailProvider;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    version: &'static str,
    email_provider: &'static str,
    email_available: bool,
}

/// GET /health
///
/// Vérifie l'état de santé du service.
pub async fn health_check(
    email_provider: web::Data<Arc<dyn EmailProvider>>,
) -> HttpResponse {
    let email_available = email_provider.health_check().await;

    let response = HealthResponse {
        status: if email_available { "healthy" } else { "degraded" },
        version: env!("CARGO_PKG_VERSION"),
        email_provider: email_provider.provider_name(),
        email_available,
    };

    if email_available {
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::ServiceUnavailable().json(response)
    }
}
