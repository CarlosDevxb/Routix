#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "rol_usuario", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum RolUsuario {
    Administrador,
    Profesor,
    Alumno,
    Tecnico,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "estado_dispositivo", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EstadoDispositivo {
    Activo,
    Mantenimiento,
    Baja,
    Extraviado,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "estado_incidente")]
pub enum EstadoIncidente {
    #[serde(rename = "abierto")]
    #[sqlx(rename = "abierto")]
    Abierto,
    #[serde(rename = "en_revision")]
    #[sqlx(rename = "en_revision")]
    EnRevision,
    #[serde(rename = "resuelto")]
    #[sqlx(rename = "resuelto")]
    Resuelto,
    #[serde(rename = "cerrado")]
    #[sqlx(rename = "cerrado")]
    Cerrado,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "severidad_incidente", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum SeveridadIncidente {
    Baja,
    Media,
    Alta,
    Critica,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "dia_semana")]
pub enum DiaSemana {
    #[serde(rename = "lunes")]
    #[sqlx(rename = "lunes")]
    Lunes,
    #[serde(rename = "martes")]
    #[sqlx(rename = "martes")]
    Martes,
    #[serde(rename = "miercoles")]
    #[sqlx(rename = "miercoles")]
    Miercoles,
    #[serde(rename = "jueves")]
    #[sqlx(rename = "jueves")]
    Jueves,
    #[serde(rename = "viernes")]
    #[sqlx(rename = "viernes")]
    Viernes,
    #[serde(rename = "sabado")]
    #[sqlx(rename = "sabado")]
    Sabado,
}
