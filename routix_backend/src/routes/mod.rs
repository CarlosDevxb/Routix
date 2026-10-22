use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

pub mod auth;
pub mod health;
pub mod usuarios;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(raiz))
        .route("/health", get(health::health_check))
        .route("/api/health", get(health::health_check))
        // Rutas de autenticación
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/me", get(auth::me))
        // Rutas de usuarios
        .route("/api/usuarios", post(usuarios::registrar_usuario).get(usuarios::listar_usuarios))
        .route("/api/usuarios/{id}", get(usuarios::obtener_usuario_por_id))
        .with_state(state)
}

async fn raiz() -> &'static str {
    "API Routix funcionando correctamente"
}
