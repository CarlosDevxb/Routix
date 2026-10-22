#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::enums::RolUsuario;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Usuario {
    pub id: i32,
    pub matricula: String,
    pub nombre: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub rol: Option<RolUsuario>,
    pub creado_en: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CrearUsuarioDTO {
    pub matricula: String,
    pub nombre: String,
    pub email: String,
    pub password: String,
    pub rol: Option<RolUsuario>,
}
