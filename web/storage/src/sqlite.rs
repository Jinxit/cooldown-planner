use crate::get_system_time;
use crate::store::{Connection, StoreKey};
use async_trait::async_trait;
use serde_json::Value;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::time::Duration;

#[derive(Clone)]
pub struct SqLiteConnection {
    pool: Pool<Sqlite>,
}

impl SqLiteConnection {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(64)
            .connect_with(
                SqliteConnectOptions::from_str(url)
                    .unwrap()
                    .create_if_missing(true)
                    // Use write-ahead logging journal, with a less strict sync
                    // mode. This presents a small risk of data loss, but no risk of
                    // corruption.
                    .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
                    .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
                    // shared cache with read_uncommitted drops the isolation level for better performance
                    .shared_cache(true)
                    .pragma("read_uncommitted", "true"),
            )
            .await?;

        #[cfg(debug_assertions)]
        sqlx::query!("DELETE FROM _sqlx_migrations;")
            .execute(&pool)
            .await?;

        #[cfg(debug_assertions)]
        sqlx::query!("DROP TABLE IF EXISTS store;")
            .execute(&pool)
            .await?;

        sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl Connection for SqLiteConnection {
    async fn get(&self, key: &StoreKey) -> Option<Value> {
        let now = get_system_time() as i64;
        let mut conn = self.pool.acquire().await.unwrap();
        let key_type = hash(&key.key_type);
        let value_type = hash(&key.value_type);

        sqlx::query_scalar!(
            r#"SELECT value
            FROM store
            WHERE key = ?
            AND key_type = ?
            AND value_type = ?
            AND (live_until IS NULL OR live_until > ?)
            ;"#,
            key.key,
            key_type,
            value_type,
            now
        )
        .fetch_one(&mut conn)
        .await
        .ok()
        .and_then(|s: String| serde_json::from_str(&s).ok())
    }

    async fn put(&self, key: &StoreKey, value: Value, ttl: Option<Duration>) {
        let live_until = ttl.map(|t| (t.as_secs() + get_system_time()) as i64);
        let mut conn = self.pool.acquire().await.unwrap();
        let value = serde_json::to_string(&value).unwrap();
        let key_type = hash(&key.key_type);
        let value_type = hash(&key.value_type);

        sqlx::query!(
            r#"INSERT INTO store (
                key,
                key_type,
                value_type,
                value,
                live_until
            ) VALUES($1, $2, $3, $4, $5)
            ON CONFLICT(key, key_type, value_type)
            DO UPDATE SET
                value = $4,
                live_until = $5
            ;"#,
            key.key,
            key_type,
            value_type,
            value,
            live_until
        )
        .execute(&mut conn)
        .await
        .unwrap();
    }

    async fn delete(&self, key: &StoreKey) {
        let mut conn = self.pool.acquire().await.unwrap();
        let key_type = hash(&key.key_type);
        let value_type = hash(&key.value_type);

        sqlx::query!(
            r#"DELETE FROM store
            WHERE key = $2
            AND key_type = $3
            AND value_type = $4
            ;"#,
            key.key,
            key_type,
            value_type
        )
        .execute(&mut conn)
        .await
        .unwrap();
    }
}

fn hash<T: Hash>(t: &T) -> i64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish() as i64
}
