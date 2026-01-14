mod config;
mod db;
mod email;
mod models;
mod handlers;
mod errors;

use actix_web::{web, App, HttpServer, middleware};
use sqlx::sqlite::SqlitePool;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialiser le logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    dotenv::dotenv().ok();
    
    // PERSONNALISER : Ajouter ici d'autres logs si nécessaire
    log::info!("🚀 Démarrage du serveur de transfert de contacts...");
    
    // Initialiser la base de données
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL doit être défini");
    
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Impossible de se connecter à la base de données");
    
    // Exécuter les migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Impossible d'exécuter les migrations");
    
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("SERVER_PORT doit être un nombre");
    
    log::info!("📡 Serveur écoute sur {}:{}", host, port);
    
    let server_addr = format!("{}:{}", host, port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            // PERSONNALISER : Ajouter ici vos middlewares (CORS, authentification, etc.)
            .configure(handlers::routes)
    })
    .bind(&server_addr)?
    .run()
    .await
}
