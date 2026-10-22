use axum::{extract::State, Json};

use crate::{
    error::{AppError, AppResult},
    extractors::AuthUser,
    models::{LoginDTO, LoginRespuestaDTO, Usuario},
    state::AppState,
    utils::{
        crypto::verify_password,
        jwt::generar_token,
    },
};

// POST /api/auth/login
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginDTO>,
) -> AppResult<Json<LoginRespuestaDTO>> {
    let identificador = payload.identificador.trim();
    if identificador.is_empty() || payload.password.is_empty() {
        return Err(AppError::BadRequest(
            "Debes ingresar identificador (matrícula o correo) y contraseña".into(),
        ));
    }

    // Buscar por correo o por matrícula
    let usuario = sqlx::query_as!(
        Usuario,
        r#"
        SELECT id, matricula, nombre, email, password_hash, rol as "rol: _", creado_en
        FROM usuarios
        WHERE LOWER(email) = LOWER($1) OR matricula = $1
        "#,
        identificador
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Credenciales incorrectas".into()))?;

    // Verificar contraseña con Argon2
    let pass_valida = verify_password(&payload.password, &usuario.password_hash)?;
    if !pass_valida {
        return Err(AppError::Unauthorized("Credenciales incorrectas".into()));
    }

    // Generar Token JWT
    let token = generar_token(&usuario, &state.jwt_secret)?;

    Ok(Json(LoginRespuestaDTO {
        token,
        tipo_token: "Bearer".into(),
        usuario,
    }))
}

// GET /api/auth/me (Ruta protegida)
pub async fn me(
    auth: AuthUser,
    State(state): State<AppState>,
) -> AppResult<Json<Usuario>> {
    let usuario = sqlx::query_as!(
        Usuario,
        r#"
        SELECT id, matricula, nombre, email, password_hash, rol as "rol: _", creado_en
        FROM usuarios
        WHERE id = $1
        "#,
        auth.0.sub
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Usuario ya no existe en el sistema".into()))?;

    Ok(Json(usuario))
}

