use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;
mod extractors;
mod models;
mod routes;
mod state;
mod utils;

use routes::create_router;
use state::AppState;

#[tokio::main]
async fn main() {
    // 1. Cargar variables de entorno desde .env
    dotenv().ok();

    // 2. Inicializar el sistema de logs con tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "routix_backend=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = env::var("DATABASE_URL").expect("Falta la variable DATABASE_URL en el archivo .env");
    let jwt_secret = env::var("JWT_SECRET").expect("Falta la variable JWT_SECRET en el archivo .env");

    // 3. Pool de conexiones PostgreSQL
    tracing::info!("Conectando a la base de datos PostgreSQL...");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("❌ Error crítico: no se pudo conectar a la base de datos PostgreSQL.");

    tracing::info!("✅ Conexión a PostgreSQL establecida exitosamente.");

    // 4. Configuración de CORS para el frontend (SolidJS / Vite)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 5. Estado y Enrutador con middlewares
    let state = AppState::new(pool, jwt_secret);
    let app = create_router(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // 6. Servidor TCP
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("❌ No se pudo enlazar el puerto {}: {}", addr, e));

    tracing::info!("🚀 Servidor Routix activo en http://localhost:{}", port);
    axum::serve(listener, app)
        .await
        .expect("Error al ejecutar el servidor HTTP");
}
