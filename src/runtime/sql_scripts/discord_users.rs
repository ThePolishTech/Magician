use sqlx::{
    sqlite::{
        SqlitePool, SqliteQueryResult
    },
    Error
};

pub async fn get_by_user_id(database_conn_pool: &SqlitePool, user_id: u64) -> Result<Option<sqlx::sqlite::SqliteRow>, Error> {
    sqlx::query("SELECT * FROM DiscordUsers WHERE pk_discordID = $1")
        .bind(user_id as i64)
        .fetch_optional(database_conn_pool)
        .await
}

pub async fn add_user(database_conn_pool: &SqlitePool, user_id: u64) -> Result<SqliteQueryResult, Error> {
    sqlx::query("INSERT INTO DiscordUsers VALUES ($1, NULL);")
        .bind(user_id as i64)   // SQLite does not implement unsigned integers, but we can just
                                // safely cast to i64 here
        .execute(database_conn_pool)
        .await
}
