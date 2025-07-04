use sqlx::sqlite::SqlitePool;

pub struct RuntimeClient {
    pub database_connection: SqlitePool
}
