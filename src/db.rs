use crate::errors::AppError;
use crate::models::{Contact, CreateContactRequest};
use sqlx::{sqlite::SqlitePool, Row};
use chrono::Utc;
use uuid::Uuid;

pub struct ContactRepository {
    pool: SqlitePool,
}

impl ContactRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // PERSONNALISER : Adapter les champs selon votre schéma
    pub async fn create(&self, req: &CreateContactRequest) -> Result<Contact, AppError> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO contacts (id, first_name, last_name, email, phone, company, sector, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&id)
        .bind(&req.first_name)
        .bind(&req.last_name)
        .bind(&req.email)
        .bind(&req.phone)
        .bind(&req.company)
        .bind(&req.sector)
        .bind("pending")
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(Contact {
            id,
            first_name: req.first_name.clone(),
            last_name: req.last_name.clone(),
            email: req.email.clone(),
            phone: req.phone.clone(),
            company: req.company.clone(),
            sector: req.sector.clone(),
            status: "pending".to_string(),
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Contact, AppError> {
        sqlx::query_as::<_, Contact>(
            "SELECT id, first_name, last_name, email, phone, company, sector, status, created_at, updated_at FROM contacts WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Contact {} non trouvé", id)))
    }

    pub async fn get_all(&self) -> Result<Vec<Contact>, AppError> {
        Ok(sqlx::query_as::<_, Contact>(
            "SELECT id, first_name, last_name, email, phone, company, sector, status, created_at, updated_at FROM contacts ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn get_by_ids(&self, ids: &[String]) -> Result<Vec<Contact>, AppError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        // PERSONNALISER : Gérer les IDs de manière sécurisée
        let placeholders = (0..ids.len())
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");

        let query_str = format!(
            "SELECT id, first_name, last_name, email, phone, company, sector, status, created_at, updated_at FROM contacts WHERE id IN ({})",
            placeholders
        );

        let mut query = sqlx::query_as::<_, Contact>(&query_str);
        for id in ids {
            query = query.bind(id);
        }

        Ok(query.fetch_all(&self.pool).await?)
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM contacts WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_bulk(&self, ids: &[String]) -> Result<u64, AppError> {
        if ids.is_empty() {
            return Ok(0);
        }

        let placeholders = (0..ids.len())
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");

        let query_str = format!("DELETE FROM contacts WHERE id IN ({})", placeholders);

        let mut query = sqlx::query(&query_str);
        for id in ids {
            query = query.bind(id);
        }

        let result = query.execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn update_status(&self, id: &str, status: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE contacts SET status = ?, updated_at = ? WHERE id = ?")
            .bind(status)
            .bind(Utc::now())
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
