use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};

use crate::{
    error::AppError,
    state::AppState,
    utils::jwt::{verificar_token, Claims},
};

pub struct AuthUser(pub Claims);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| {
                AppError::Unauthorized("Cabecera Authorization (Bearer token) no proporcionada".to_string())
            })?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized(
                "Formato de token inválido. El formato esperado es: Bearer <token>".to_string(),
            ));
        }

        let token = &auth_header[7..].trim();
        let claims = verificar_token(token, &state.jwt_secret)?;

        Ok(AuthUser(claims))
    }
}

