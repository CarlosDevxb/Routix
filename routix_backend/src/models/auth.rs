use serde::{Deserialize, Serialize};

use super::usuario::Usuario;

#[derive(Debug, Deserialize)]
pub struct LoginDTO {
    /// Permite ingresar con matrícula o con correo electrónico
    pub identificador: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginRespuestaDTO {
    pub token: String,
    pub tipo_token: String,
    pub usuario: Usuario,
}

