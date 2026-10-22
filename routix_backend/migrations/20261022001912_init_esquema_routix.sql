-- Add migration script here
-- 1. Enumeradores de estado, roles y tiempo
CREATE TYPE rol_usuario AS ENUM ('administrador', 'profesor', 'alumno', 'tecnico');
CREATE TYPE estado_dispositivo AS ENUM ('activo', 'mantenimiento', 'baja', 'extraviado');
CREATE TYPE estado_incidente AS ENUM ('abierto', 'en_revision', 'resuelto', 'cerrado');
CREATE TYPE severidad_incidente AS ENUM ('baja', 'media', 'alta', 'critica');
CREATE TYPE dia_semana AS ENUM ('lunes', 'martes', 'miercoles', 'jueves', 'viernes', 'sabado');

-- 2. Usuarios 
CREATE TABLE usuarios (
    id SERIAL PRIMARY KEY,
    matricula VARCHAR(50) UNIQUE NOT NULL,
    nombre VARCHAR(100) NOT NULL,
    email VARCHAR(150) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    rol rol_usuario DEFAULT 'alumno',
    creado_en TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 3. Plantilla de Horarios del Laboratorio
CREATE TABLE horarios_clase (
    id SERIAL PRIMARY KEY,
    materia VARCHAR(150) NOT NULL,              
    profesor_id INT NOT NULL REFERENCES usuarios(id), 
    dia dia_semana NOT NULL,
    hora_inicio TIME NOT NULL,                  
    hora_fin TIME NOT NULL                      
);

-- 4. Proveedores y Estaciones (Topología)
CREATE TABLE proveedores (
    id SERIAL PRIMARY KEY,
    nombre_empresa VARCHAR(150) NOT NULL,
    contacto_principal VARCHAR(100),
    telefono VARCHAR(20),
    email VARCHAR(150),
    notas TEXT
);

CREATE TABLE estaciones_trabajo (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(50) UNIQUE NOT NULL -- Ej. "Isla 1", "Mesa 4", "Rack Principal"
);

-- 5. Catálogo de Hardware (3NF)
CREATE TABLE categorias_dispositivos (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL UNIQUE
);

CREATE TABLE modelos_dispositivos (
    id SERIAL PRIMARY KEY,
    categoria_id INT NOT NULL REFERENCES categorias_dispositivos(id),
    proveedor_id INT REFERENCES proveedores(id),
    marca VARCHAR(50) NOT NULL,
    modelo VARCHAR(100) NOT NULL,
    UNIQUE(marca, modelo)
);

CREATE TABLE catalogo_caracteristicas (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL UNIQUE
);

CREATE TABLE caracteristicas_modelo (
    modelo_id INT NOT NULL REFERENCES modelos_dispositivos(id) ON DELETE CASCADE,
    caracteristica_id INT NOT NULL REFERENCES catalogo_caracteristicas(id) ON DELETE CASCADE,
    valor VARCHAR(100) NOT NULL,
    PRIMARY KEY (modelo_id, caracteristica_id)
);

-- 6. Dispositivos Físicos
CREATE TABLE dispositivos_fisicos (
    id SERIAL PRIMARY KEY,
    modelo_id INT NOT NULL REFERENCES modelos_dispositivos(id),
    estacion_trabajo_id INT REFERENCES estaciones_trabajo(id) ON DELETE SET NULL,
    numero_serie VARCHAR(100) UNIQUE NOT NULL,
    etiqueta_inventario VARCHAR(50) UNIQUE NOT NULL,
    estado estado_dispositivo DEFAULT 'activo',
    fecha_adquisicion DATE
);

-- 7. BITÁCORA DE CLASES (Registro real operado por el profesor)
CREATE TABLE bitacora_clases (
    id SERIAL PRIMARY KEY,
    horario_id INT NOT NULL REFERENCES horarios_clase(id),
    fecha DATE NOT NULL DEFAULT CURRENT_DATE, -- Qué día exacto se dio la clase
    observaciones_profesor TEXT,              -- Notas generales de la sesión
    creado_en TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 7.1 Relación de islas/estaciones utilizadas en esa clase
CREATE TABLE estaciones_usadas_clase (
    bitacora_id INT NOT NULL REFERENCES bitacora_clases(id) ON DELETE CASCADE,
    estacion_trabajo_id INT NOT NULL REFERENCES estaciones_trabajo(id),
    PRIMARY KEY (bitacora_id, estacion_trabajo_id)
);

-- 8. Sistema de Reportes e Incidentes
CREATE TABLE incidentes (
    id SERIAL PRIMARY KEY,
    dispositivo_id INT NOT NULL REFERENCES dispositivos_fisicos(id),
    reportado_por INT NOT NULL REFERENCES usuarios(id), 
    bitacora_id INT REFERENCES bitacora_clases(id),        -- <-- Vincula el daño a la clase exacta y al profesor
    titulo VARCHAR(150) NOT NULL,
    descripcion TEXT NOT NULL,
    severidad severidad_incidente DEFAULT 'media',
    estado estado_incidente DEFAULT 'abierto',
    creado_en TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    resuelto_en TIMESTAMPTZ,
    notas_resolucion TEXT
);