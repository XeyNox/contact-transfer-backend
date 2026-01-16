//! Backend API SMP Moules
//!
//! API REST pour l'application Android de collecte de contacts
//! lors des salons professionnels.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                      HTTP Layer                              │
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
//! │  │   /health   │  │/api/export  │  │ /api/send-history   │  │
//! │  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────┘  │
//! └─────────┼────────────────┼───────────────────┼──────────────┘
//!           │                │                   │
//! ┌─────────┴────────────────┴───────────────────┴──────────────┐
//! │                     Handlers (thin)                          │
//! │  - Validation                                                │
//! │  - Auth check                                                │
//! │  - Response formatting                                       │
//! └─────────────────────────────┬────────────────────────────────┘
//!                               │
//! ┌─────────────────────────────┴────────────────────────────────┐
//! │                     Domain Layer                             │
//! │  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐   │
//! │  │ContactFiche  │  │    Email     │  │   Templates      │   │
//! │  └──────────────┘  └──────────────┘  └──────────────────┘   │
//! └─────────────────────────────┬────────────────────────────────┘
//!                               │
//! ┌─────────────────────────────┴────────────────────────────────┐
//! │                   Email Provider (trait)                     │
//! │                           │                                  │
//! │              ┌────────────┴────────────┐                     │
//! │              │     ResendProvider      │                     │
//! │              └─────────────────────────┘                     │
//! └──────────────────────────────────────────────────────────────┘
//! ```

mod config;
mod domain;
mod email;
mod handlers;
mod middleware;

use actix_web::{web, App, HttpServer, middleware as actix_middleware};
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::config::AppConfig;
use crate::email::{EmailProvider, ResendProvider};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Initialiser le logging
    init_tracing();

    // 2. Charger la configuration
    let config = AppConfig::from_env().expect("Erreur de configuration");
    let config = Arc::new(config);

    info!(
        host = %config.server.host,
        port = %config.server.port,
        "Démarrage du serveur SMP Backend"
    );

    // 3. Créer le provider email
    let email_provider: Arc<dyn EmailProvider> = Arc::new(ResendProvider::new(&config.email));

    info!(
        provider = %email_provider.provider_name(),
        "Provider email initialisé"
    );

    // 4. Démarrer le serveur
    let server_config = config.clone();
    
    HttpServer::new(move || {
        App::new()
            // Middleware
            .wrap(actix_middleware::Logger::default())
            .wrap(actix_middleware::Compress::default())
            
            // State partagé
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(email_provider.clone()))
            
            // Configuration JSON
            .app_data(web::JsonConfig::default().limit(10 * 1024 * 1024)) // 10MB limit
            
            // Routes
            .route("/health", web::get().to(handlers::health_check))
            .route("/api/export-fiches", web::post().to(handlers::export_fiches))
            .route("/api/send-history-email", web::post().to(handlers::send_history_email))
    })
    .bind((server_config.server.host.as_str(), server_config.server.port))?
    .run()
    .await
}

/// Initialise le système de logging/tracing
fn init_tracing() {
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(true)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .compact()
        .init();
}
