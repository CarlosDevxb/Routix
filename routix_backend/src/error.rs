use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Recurso no encontrado: {0}")]
    NotFound(String),

    #[error("Petición inválida: {0}")]
    BadRequest(String),

    #[error("Conflicto: {0}")]
    Conflict(String),

    #[error("No autorizado: {0}")]
    Unauthorized(String),

    #[error("Acceso prohibido: {0}")]
    Forbidden(String),

    #[error("Error de criptografía/hash: {0}")]
    PasswordError(String),

    #[error("Error interno del servidor: {0}")]
    Internal(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            AppError::PasswordError(msg) => {
                tracing::error!("Error de seguridad/contraseña: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error al procesar credenciales de seguridad".to_string(),
                )
            }
            AppError::Database(err) => {
                tracing::error!("Error de SQLx: {:?}", err);
                match err {
                    sqlx::Error::Database(db_err) => {
                        // 23505 = unique_violation
                        if db_err.code().as_deref() == Some("23505") {
                            (
                                StatusCode::CONFLICT,
                                format!(
                                    "Ya existe un registro con esos datos únicos: {}",
                                    db_err.message()
                                ),
                            )
                        } else if db_err.code().as_deref() == Some("23503") {
                            // 23503 = foreign_key_violation
                            (
                                StatusCode::BAD_REQUEST,
                                format!(
                                    "Referencia no válida a otro recurso: {}",
                                    db_err.message()
                                ),
                            )
                        } else {
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "Error interno en la base de datos".to_string(),
                            )
                        }
                    }
                    sqlx::Error::RowNotFound => (
                        StatusCode::NOT_FOUND,
                        "El registro solicitado no fue encontrado".to_string(),
                    ),
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Error inesperado al consultar la base de datos".to_string(),
                    ),
                }
            }
            AppError::Internal(msg) => {
                tracing::error!("Error interno: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error interno del servidor".to_string(),
                )
            }
        };

        let body = Json(json!({
            "success": false,
            "status": status.as_u16(),
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
