use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    error::{AppError, AppResult},
    extractors::AuthUser,
    models::{CrearUsuarioDTO, RolUsuario, Usuario},
    state::AppState,
    utils::crypto::hash_password,
};

// POST /api/usuarios (Registro inicial o creación administrativa)
pub async fn registrar_usuario(
    State(state): State<AppState>,
    Json(payload): Json<CrearUsuarioDTO>,
) -> AppResult<(StatusCode, Json<Usuario>)> {
    // Validaciones básicas de entrada
    if payload.matricula.trim().is_empty() {
        return Err(AppError::BadRequest("La matrícula es requerida".into()));
    }
    if payload.nombre.trim().is_empty() {
        return Err(AppError::BadRequest("El nombre es requerido".into()));
    }
    if payload.email.trim().is_empty() {
        return Err(AppError::BadRequest("El correo electrónico es requerido".into()));
    }
    if payload.password.len() < 6 {
        return Err(AppError::BadRequest(
            "La contraseña debe tener al menos 6 caracteres".into(),
        ));
    }

    // Hashear la contraseña de forma segura con Argon2
    let password_hash = hash_password(&payload.password)?;

    let nuevo_usuario = sqlx::query_as!(
        Usuario,
        r#"
        INSERT INTO usuarios (matricula, nombre, email, password_hash, rol)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, matricula, nombre, email, password_hash, rol as "rol: _", creado_en
        "#,
        payload.matricula.trim(),
        payload.nombre.trim(),
        payload.email.trim().to_lowercase(),
        password_hash,
        payload.rol as Option<RolUsuario>
    )
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(nuevo_usuario)))
}

// GET /api/usuarios (Protegido por Rol: Solo Administrador, Tecnico o Profesor)
pub async fn listar_usuarios(
    auth: AuthUser,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<Usuario>>> {
    auth.0.require_role(&[
        RolUsuario::Administrador,
        RolUsuario::Tecnico,
        RolUsuario::Profesor,
    ])?;

    let usuarios = sqlx::query_as!(
        Usuario,
        r#"
        SELECT id, matricula, nombre, email, password_hash, rol as "rol: _", creado_en
        FROM usuarios
        ORDER BY id ASC
        "#
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(usuarios))
}

// GET /api/usuarios/:id (Protegido: El propio usuario, o Administrador/Técnico)
pub async fn obtener_usuario_por_id(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<Json<Usuario>> {
    if auth.0.rol != RolUsuario::Administrador
        && auth.0.rol != RolUsuario::Tecnico
        && auth.0.sub != id
    {
        return Err(AppError::Forbidden(
            "No tienes permisos para consultar datos de otro usuario".into(),
        ));
    }

    let usuario = sqlx::query_as!(
        Usuario,
        r#"
        SELECT id, matricula, nombre, email, password_hash, rol as "rol: _", creado_en
        FROM usuarios
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Usuario con ID {} no encontrado", id)))?;

    Ok(Json(usuario))
}


