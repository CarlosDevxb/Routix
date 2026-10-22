use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(db: Pool<Postgres>, jwt_secret: String) -> Self {
        Self { db, jwt_secret }
    }
}
