use crate::db::ContactRepository;
use crate::email::EmailService;
use crate::config::EmailConfig;
use crate::models::*;
use crate::errors::AppError;
use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;

// PERSONNALISER : Ajouter les routes supplémentaires selon vos besoins
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/contacts")
            .route("", web::post().to(create_contact))
            .route("", web::get().to(list_contacts))
            .route("/{id}", web::get().to(get_contact))
            .route("/{id}", web::delete().to(delete_contact))
            .route("/export/email", web::post().to(export_by_email))
            .route("/bulk-delete", web::post().to(bulk_delete))
    );
    
    cfg.route("/health", web::get().to(health_check));
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "version": "0.1.0"
    }))
}

// PERSONNALISER : Ajouter de la validation supplémentaire si nécessaire
pub async fn create_contact(
    pool: web::Data<SqlitePool>,
    req: web::Json<CreateContactRequest>,
) -> Result<HttpResponse, AppError> {
    log::info!("Création d'un nouveau contact: {} {}", req.first_name, req.last_name);
    
    // PERSONNALISER : Ajouter des validations métier
    if req.email.is_empty() {
        return Err(AppError::ValidationError("Email requis".to_string()));
    }

    let repo = ContactRepository::new(pool.get_ref().clone());
    let contact = repo.create(&req).await?;

    Ok(HttpResponse::Created().json(ApiResponse::success(
        contact,
        "Contact créé avec succès"
    )))
}

pub async fn list_contacts(
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, AppError> {
    let repo = ContactRepository::new(pool.get_ref().clone());
    let contacts = repo.get_all().await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(
        contacts,
        format!("{} contact(s) trouvé(s)", contacts.len())
    )))
}

pub async fn get_contact(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let repo = ContactRepository::new(pool.get_ref().clone());
    let contact = repo.get_by_id(&id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(
        contact,
        "Contact récupéré"
    )))
}

pub async fn delete_contact(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let repo = ContactRepository::new(pool.get_ref().clone());
    repo.delete(&id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(
        (),
        "Contact supprimé"
    )))
}

// PERSONNALISER : Adapter selon votre logique d'export
pub async fn export_by_email(
    pool: web::Data<SqlitePool>,
    req: web::Json<EmailExportRequest>,
) -> Result<HttpResponse, AppError> {
    log::info!("Demande d'export email vers: {}", req.recipient_email);

    let repo = ContactRepository::new(pool.get_ref().clone());
    let contacts = repo.get_by_ids(&req.contact_ids).await?;

    if contacts.is_empty() {
        return Err(AppError::ValidationError(
            "Aucun contact à exporter".to_string()
        ));
    }

    let email_config = EmailConfig::from_env();
    let email_service = EmailService::new(email_config);
    
    let message = email_service
        .send_contacts(&req.recipient_email, &contacts)
        .await?;

    // PERSONNALISER : Mettre à jour le statut des contacts si nécessaire
    for contact in &contacts {
        let _ = repo.update_status(&contact.id, "sent").await;
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success(
        EmailExportResponse {
            success: true,
            message,
            export_id: uuid::Uuid::new_v4().to_string(),
        },
        "Export réussi"
    )))
}

// PERSONNALISER : Ajouter des logs d'audit
pub async fn bulk_delete(
    pool: web::Data<SqlitePool>,
    req: web::Json<BulkDeleteRequest>,
) -> Result<HttpResponse, AppError> {
    log::info!("Suppression en masse de {} contacts", req.contact_ids.len());

    let repo = ContactRepository::new(pool.get_ref().clone());
    let deleted = repo.delete_bulk(&req.contact_ids).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(
        serde_json::json!({ "deleted": deleted }),
        format!("{} contact(s) supprimé(s)", deleted)
    )))
}
