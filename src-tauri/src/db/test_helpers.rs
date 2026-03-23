use sqlx::SqlitePool;

/// Spin up a fully migrated, in-memory SQLite pool for use in tests.
/// Each call returns an independent pool — tests are fully isolated.
pub async fn test_pool() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:")
        .await
        .expect("failed to open in-memory SQLite");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migrations failed");

    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await
        .expect("failed to enable FK constraints");

    pool
}
