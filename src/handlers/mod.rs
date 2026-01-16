//! Handlers HTTP pour les endpoints API.
//!
//! Les handlers sont minces et délèguent la logique métier
//! aux services appropriés.

mod export_fiches;
mod health;
mod history;

pub use export_fiches::export_fiches;
pub use health::health_check;
pub use history::send_history_email;
