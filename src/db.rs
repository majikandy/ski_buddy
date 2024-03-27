use chrono::{DateTime, Utc};
use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use serde::{Serialize, Deserialize};
use std::fs::File;
use log::{info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct Run {
    pub id: i64,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Turn {
    pub id: i64,
    pub run_id: i64,
    pub direction: String,
    pub parallelness: f64,
    pub closeness: f64,
    pub smoothness: String,
    pub timestamp_start: DateTime<Utc>,
    pub timestamp_end: DateTime<Utc>,
}

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    // Get database URL from environment or use default
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:ski.db".to_string());

    info!("Using database URL: {}", database_url);

    // Create the database file if it doesn't exist
    if let Some(path) = database_url.strip_prefix("sqlite:") {
        let path = path.trim_start_matches('/');  // Remove leading slash if present
        info!("Database path: {}", path);
        
        if !std::path::Path::new(path).exists() {
            info!("Creating database file at: {}", path);
            match File::create(path) {
                Ok(_) => info!("Database file created successfully"),
                Err(e) => {
                    warn!("Failed to create database file: {}", e);
                    // Continue anyway as SQLx might be able to create it
                }
            }
        } else {
            info!("Database file already exists");
        }
    }

    info!("Connecting to database...");
    let pool = SqlitePool::connect(&database_url).await?;
    info!("Database connection established");

    // Create tables if they don't exist
    info!("Creating tables if they don't exist...");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS runs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            start_time DATETIME NOT NULL,
            end_time DATETIME NOT NULL,
            path TEXT NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS turns (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            run_id INTEGER NOT NULL,
            direction TEXT NOT NULL,
            parallelness REAL NOT NULL,
            closeness REAL NOT NULL,
            smoothness TEXT NOT NULL,
            timestamp_start DATETIME NOT NULL,
            timestamp_end DATETIME NOT NULL,
            FOREIGN KEY (run_id) REFERENCES runs(id)
        )
        "#,
    )
    .execute(&pool)
    .await?;
    info!("Tables created successfully");

    // Check if the database is empty
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM runs")
        .fetch_one(&pool)
        .await?;

    if count.0 == 0 {
        info!("Database is empty, inserting initial data...");
        // Insert initial data only if the database is empty
        let run_id = insert_run(
            &pool,
            "2024-02-20T10:00:00Z",
            "2024-02-20T10:01:30Z",
            "M 0 50 C 25 25, 75 75, 100 50",
        )
        .await?;

        insert_turn(
            &pool,
            run_id,
            "left",
            0.85,
            0.92,
            "smooth",
            DateTime::parse_from_rfc3339("2024-02-20T10:00:00Z").unwrap().with_timezone(&Utc),
            DateTime::parse_from_rfc3339("2024-02-20T10:00:01.2Z").unwrap().with_timezone(&Utc),
        )
        .await?;

        insert_turn(
            &pool,
            run_id,
            "right",
            0.88,
            0.95,
            "abrupt",
            DateTime::parse_from_rfc3339("2024-02-20T10:00:01.5Z").unwrap().with_timezone(&Utc),
            DateTime::parse_from_rfc3339("2024-02-20T10:00:02.7Z").unwrap().with_timezone(&Utc),
        )
        .await?;

        insert_turn(
            &pool,
            run_id,
            "left",
            0.82,
            0.90,
            "smooth",
            DateTime::parse_from_rfc3339("2024-02-20T10:00:03.0Z").unwrap().with_timezone(&Utc),
            DateTime::parse_from_rfc3339("2024-02-20T10:00:04.1Z").unwrap().with_timezone(&Utc),
        )
        .await?;

        let run_id = insert_run(
            &pool,
            "2024-02-20T11:00:00Z",
            "2024-02-20T11:02:00Z",
            "M 0 50 C 25 75, 75 25, 100 50",
        )
        .await?;

        insert_turn(
            &pool,
            run_id,
            "right",
            0.87,
            0.93,
            "smooth",
            DateTime::parse_from_rfc3339("2024-02-20T11:00:00Z").unwrap().with_timezone(&Utc),
            DateTime::parse_from_rfc3339("2024-02-20T11:00:01.1Z").unwrap().with_timezone(&Utc),
        )
        .await?;

        insert_turn(
            &pool,
            run_id,
            "left",
            0.89,
            0.96,
            "smooth",
            DateTime::parse_from_rfc3339("2024-02-20T11:00:01.4Z").unwrap().with_timezone(&Utc),
            DateTime::parse_from_rfc3339("2024-02-20T11:00:02.3Z").unwrap().with_timezone(&Utc),
        )
        .await?;

        insert_turn(
            &pool,
            run_id,
            "right",
            0.91,
            0.94,
            "smooth",
            DateTime::parse_from_rfc3339("2024-02-20T11:00:02.6Z").unwrap().with_timezone(&Utc),
            DateTime::parse_from_rfc3339("2024-02-20T11:00:03.8Z").unwrap().with_timezone(&Utc),
        )
        .await?;

        insert_turn(
            &pool,
            run_id,
            "left",
            0.88,
            0.92,
            "abrupt",
            DateTime::parse_from_rfc3339("2024-02-20T11:00:04.0Z").unwrap().with_timezone(&Utc),
            DateTime::parse_from_rfc3339("2024-02-20T11:00:05.2Z").unwrap().with_timezone(&Utc),
        )
        .await?;

        insert_turn(
            &pool,
            run_id,
            "right",
            0.86,
            0.91,
            "smooth",
            DateTime::parse_from_rfc3339("2024-02-20T11:00:05.5Z").unwrap().with_timezone(&Utc),
            DateTime::parse_from_rfc3339("2024-02-20T11:00:06.7Z").unwrap().with_timezone(&Utc),
        )
        .await?;

        insert_turn(
            &pool,
            run_id,
            "left",
            0.90,
            0.95,
            "smooth",
            DateTime::parse_from_rfc3339("2024-02-20T11:00:07.0Z").unwrap().with_timezone(&Utc),
            DateTime::parse_from_rfc3339("2024-02-20T11:00:08.1Z").unwrap().with_timezone(&Utc),
        )
        .await?;
    } else {
        info!("Database already contains {} runs", count.0);
    }

    Ok(pool)
}

pub async fn get_runs(pool: &SqlitePool) -> Result<Vec<Run>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT id, start_time, end_time, path
        FROM runs
        ORDER BY start_time DESC
        "#,
    )
    .try_map(|row: sqlx::sqlite::SqliteRow| {
        Ok(Run {
            id: row.get("id"),
            start_time: DateTime::parse_from_rfc3339(&row.get::<String, _>("start_time"))
                .map_err(|e| sqlx::Error::ColumnDecode {
                    index: "start_time".into(),
                    source: Box::new(e),
                })?
                .with_timezone(&Utc),
            end_time: DateTime::parse_from_rfc3339(&row.get::<String, _>("end_time"))
                .map_err(|e| sqlx::Error::ColumnDecode {
                    index: "end_time".into(),
                    source: Box::new(e),
                })?
                .with_timezone(&Utc),
            path: row.get("path"),
        })
    })
    .fetch_all(pool)
    .await
}

pub async fn get_turns_for_run(pool: &SqlitePool, run_id: i64) -> Result<Vec<Turn>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT id, run_id, direction, parallelness, closeness, smoothness, timestamp_start, timestamp_end
        FROM turns
        WHERE run_id = ?
        ORDER BY timestamp_start ASC
        "#,
    )
    .bind(run_id)
    .try_map(|row: sqlx::sqlite::SqliteRow| {
        Ok(Turn {
            id: row.get("id"),
            run_id: row.get("run_id"),
            direction: row.get("direction"),
            parallelness: row.get("parallelness"),
            closeness: row.get("closeness"),
            smoothness: row.get("smoothness"),
            timestamp_start: DateTime::parse_from_rfc3339(&row.get::<String, _>("timestamp_start"))
                .map_err(|e| sqlx::Error::ColumnDecode {
                    index: "timestamp_start".into(),
                    source: Box::new(e),
                })?
                .with_timezone(&Utc),
            timestamp_end: DateTime::parse_from_rfc3339(&row.get::<String, _>("timestamp_end"))
                .map_err(|e| sqlx::Error::ColumnDecode {
                    index: "timestamp_end".into(),
                    source: Box::new(e),
                })?
                .with_timezone(&Utc),
        })
    })
    .fetch_all(pool)
    .await
}

pub async fn insert_run(
    pool: &SqlitePool,
    start_time: &str,
    end_time: &str,
    path: &str,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO runs (start_time, end_time, path)
        VALUES (?, ?, ?)
        RETURNING id
        "#,
    )
    .bind(start_time)
    .bind(end_time)
    .bind(path)
    .fetch_one(pool)
    .await?;

    Ok(result.get("id"))
}

pub async fn insert_turn(
    pool: &SqlitePool,
    run_id: i64,
    direction: &str,
    parallelness: f64,
    closeness: f64,
    smoothness: &str,
    timestamp_start: DateTime<Utc>,
    timestamp_end: DateTime<Utc>,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO turns (run_id, direction, parallelness, closeness, smoothness, timestamp_start, timestamp_end)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        RETURNING id
        "#,
    )
    .bind(run_id)
    .bind(direction)
    .bind(parallelness)
    .bind(closeness)
    .bind(smoothness)
    .bind(timestamp_start.to_rfc3339())
    .bind(timestamp_end.to_rfc3339())
    .fetch_one(pool)
    .await?;

    Ok(result.get("id"))
} 