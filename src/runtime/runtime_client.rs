#![allow(dead_code)]

use sqlx::sqlite::SqlitePool;

pub struct RuntimeClient {
    pub database_connection: SqlitePool,
    pub wakeup_channel_id: u64
}
