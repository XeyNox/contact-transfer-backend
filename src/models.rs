use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// PERSONNALISER : Adapter ces structures selon votre modèle de données
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub sector: Option<String>,
    pub status: String, // pending, sent, failed, etc.
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContactRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub sector: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactResponse {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub sector: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

// PERSONNALISER : Adapter selon vos besoins d'envoi d'email
#[derive(Debug, Serialize, Deserialize)]
pub struct EmailExportRequest {
    pub recipient_email: String,
    pub contact_ids: Vec<String>,
    pub include_photo: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailExportResponse {
    pub success: bool,
    pub message: String,
    pub export_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkDeleteRequest {
    pub contact_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: message.into(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            message: message.into(),
        }
    }
}
