use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    pub db_pool: PgPool,
}
