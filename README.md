# Routix 🚀

Sistema integral para la gestión de inventario, control de uso de estaciones y soporte técnico (tickets de incidentes) para el Laboratorio de Arquitectura de Computadoras.

---

## 🛠️ Stack Tecnológico

- **Backend:** Rust, Axum, Tokio, SQLx (consultas verificadas en tiempo de compilación), Chrono, Argon2id (hashing de contraseñas), JSON Web Tokens (JWT).
- **Base de Datos:** PostgreSQL 16 con tipos `ENUM` nativos e integridad referencial (3NF).
- **Frontend:** SolidJS, TypeScript, Vite, Tailwind CSS, `@solidjs/router`.
- **Infraestructura:** Docker & Docker Compose (orquestación multi-contenedor).

---

## 📁 Estructura del Repositorio (Monorepo)

```text
Routix/
├── docker-compose.yml           # Orquestación de DB, Backend y Frontend
├── .env.example                 # Plantilla de variables de entorno
├── .gitignore                   # Exclusiones de Git
├── routix_backend/              # API REST en Rust
│   ├── Cargo.toml
│   ├── Dockerfile               # Multi-stage build con SQLX_OFFLINE
│   ├── .sqlx/                   # Metadata de consultas SQLx preparadas
│   ├── migrations/              # Esquema DDL en PostgreSQL
│   └── src/
│       ├── main.rs              # Punto de entrada, CORS, tracing y middlewares
│       ├── state.rs             # AppState compartido
│       ├── error.rs             # AppError e IntoResponse unificado
│       ├── extractors/          # Extractor AuthUser (JWT y RBAC)
│       ├── models/              # Modelos de dominio y enums
│       ├── routes/              # Handlers y subrutas (Auth, Usuarios, Health)
│       └── utils/               # Criptografía Argon2 y JWT
└── routix_frontend/             # SPA Reactiva en SolidJS
    ├── package.json
    ├── Dockerfile               # Multi-stage (Node builder -> Nginx production)
    ├── nginx.conf               # Configuración de proxy inverso y SPA
    ├── vite.config.ts           # Configuración de Vite con Tailwind y Proxy
    └── src/
        ├── api/                 # Cliente fetch con Bearer Token automático
        ├── context/             # AuthContext y señal global de usuario
        ├── pages/               # Vistas (Dashboard, Login, etc.)
        └── components/          # Componentes reusables (Navbar)
```

---

## 🚀 Despliegue con Docker Compose

Para levantar todo el stack (PostgreSQL, Backend y Frontend con Nginx) con un solo comando:

```bash
docker compose up --build
```

- **Frontend:** [http://localhost:5173](http://localhost:5173)
- **Backend API:** [http://localhost:3000](http://localhost:3000)
- **PostgreSQL:** `localhost:5432`

---

## 💻 Desarrollo Local (Sin Docker)

### 1. Requisitos
- Rust 1.85+ (`cargo`)
- Node.js 22+ y npm
- PostgreSQL corriendo localmente con la base de datos `routix_db`

### 2. Backend
```bash
cd routix_backend
cargo run
```

### 3. Frontend
```bash
cd routix_frontend
npm install
npm run dev
```
Accede al frontend de desarrollo en [http://localhost:5173](http://localhost:5173).

---

## 🔐 Seguridad y Autenticación (RBAC)

- Las contraseñas se almacenan cifradas con **Argon2id**.
- Las rutas protegidas requieren cabecera `Authorization: Bearer <token>`.
- Control de acceso basado en roles: `administrador`, `profesor`, `alumno` y `tecnico`.

