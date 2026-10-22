use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    error::{AppError, AppResult},
    models::{RolUsuario, Usuario},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32, // ID de usuario
    pub matricula: String,
    pub nombre: String,
    pub email: String,
    pub rol: RolUsuario,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn require_role(&self, allowed_roles: &[RolUsuario]) -> AppResult<()> {
        if allowed_roles.contains(&self.rol) {
            Ok(())
        } else {
            Err(AppError::Forbidden(format!(
                "Tu rol '{:?}' no cuenta con permisos suficientes para esta operación",
                self.rol
            )))
        }
    }
}

pub fn generar_token(usuario: &Usuario, secret: &str) -> AppResult<String> {
    let rol = usuario.rol.unwrap_or(RolUsuario::Alumno);
    let now = Utc::now();
    let exp = now + Duration::hours(24);

    let claims = Claims {
        sub: usuario.id,
        matricula: usuario.matricula.clone(),
        nombre: usuario.nombre.clone(),
        email: usuario.email.clone(),
        rol,
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("Error generando JWT: {}", e)))
}

pub fn verificar_token(token: &str, secret: &str) -> AppResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Unauthorized(format!("Token inválido o expirado: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_generation_and_verification() {
        let secret = "mi_clave_secreta_para_pruebas_123456789";
        let usuario = Usuario {
            id: 42,
            matricula: "MAT123".into(),
            nombre: "Juan Perez".into(),
            email: "juan@example.com".into(),
            password_hash: "hash_ignorado".into(),
            rol: Some(RolUsuario::Profesor),
            creado_en: None,
        };

        let token = generar_token(&usuario, secret).expect("Generación de token falló");
        let claims = verificar_token(&token, secret).expect("Verificación de token falló");

        assert_eq!(claims.sub, 42);
        assert_eq!(claims.matricula, "MAT123");
        assert_eq!(claims.rol, RolUsuario::Profesor);

        // Prueba de roles permitidos
        assert!(claims.require_role(&[RolUsuario::Profesor, RolUsuario::Administrador]).is_ok());
        assert!(claims.require_role(&[RolUsuario::Alumno]).is_err());

        // Prueba con token manipulado
        assert!(verificar_token("token_invalido", secret).is_err());
    }
}

