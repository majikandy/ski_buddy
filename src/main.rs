mod db;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use std::sync::Arc;
use sqlx::sqlite::SqlitePool;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use env_logger;
use log::{info, error};

#[derive(Serialize)]
struct ApiResponse<T, L> {
    #[serde(rename = "_links")]
    links: L,
    data: T,
}

#[derive(Serialize)]
struct Links {
    #[serde(rename = "self")]
    self_link: Link,
    #[serde(rename = "runs")]
    runs_link: Link,
    #[serde(rename = "root")]
    root_link: Link,
    #[serde(rename = "create_run")]
    create_run_link: Link,
    #[serde(rename = "create_turn")]
    create_turn_link: Link,
}

#[derive(Serialize)]
struct Link {
    href: String,
    method: String,
}

#[derive(Serialize)]
struct Data {
    message: String,
}

#[derive(Serialize)]
struct RunData {
    runs: Vec<RunWithTurns>,
}

#[derive(Serialize)]
struct RunWithTurns {
    id: i64,
    start_time: String,
    end_time: String,
    turns: Vec<Turn>,
}

#[derive(Serialize)]
struct Turn {
    direction: String,
    parallelness: f64,
    closeness: f64,
    smoothness: String,
    timestamp_start: String,
    timestamp_end: String,
}

#[derive(Deserialize)]
struct CreateRunRequest {
    start_time: String,
    end_time: String,
}

#[derive(Deserialize)]
struct CreateTurnRequest {
    run_id: i64,
    direction: String,
    parallelness: f64,
    closeness: f64,
    smoothness: String,
    timestamp_start: String,
    timestamp_end: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting server...");
    
    // Initialize the database
    info!("Initializing database...");
    let pool = match db::init_db().await {
        Ok(pool) => {
            info!("Database initialized successfully");
            pool
        },
        Err(e) => {
            error!("Failed to initialize database: {}", e);
            return Ok(());  // Exit gracefully instead of panicking
        }
    };
    
    let pool = Arc::new(pool);
    info!("Starting HTTP server on 0.0.0.0:8080");

    HttpServer::new(move || {
        info!("Configuring server instance");
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173") // Vite's default development port
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type"]);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .route("/", web::get().to(index))
                    .route("/runs", web::get().to(runs))
                    .route("/runs", web::post().to(create_run))
                    .route("/turns", web::get().to(turns_get))
                    .route("/turns", web::post().to(create_turn))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn index() -> impl Responder {
    let response = ApiResponse {
        links: Links {
            self_link: Link {
                href: "/api".to_string(),
                method: "GET".to_string(),
            },
            runs_link: Link {
                href: "/api/runs".to_string(),
                method: "GET".to_string(),
            },
            root_link: Link {
                href: "/".to_string(),
                method: "GET".to_string(),
            },
            create_run_link: Link {
                href: "/api/runs".to_string(),
                method: "POST".to_string(),
            },
            create_turn_link: Link {
                href: "/api/turns".to_string(),
                method: "POST".to_string(),
            },
        },
        data: Data {
            message: "Hello, world!".to_string(),
        },
    };
    web::Json(serde_json::to_value(response).unwrap())
}

async fn runs(pool: web::Data<Arc<SqlitePool>>) -> impl Responder {
    let db_runs = db::get_runs(&pool).await.expect("Failed to fetch runs");
    
    let mut runs = Vec::new();
    for run in db_runs {
        let turns = db::get_turns_for_run(&pool, run.id)
            .await
            .expect("Failed to fetch turns");
            
        let run_with_turns = RunWithTurns {
            id: run.id,
            start_time: run.start_time.to_rfc3339(),
            end_time: run.end_time.to_rfc3339(),
            turns: turns
                .into_iter()
                .map(|t| Turn {
                    direction: t.direction,
                    parallelness: t.parallelness,
                    closeness: t.closeness,
                    smoothness: t.smoothness,
                    timestamp_start: t.timestamp_start.to_rfc3339(),
                    timestamp_end: t.timestamp_end.to_rfc3339(),
                })
                .collect(),
        };
        runs.push(run_with_turns);
    }

    let response = ApiResponse {
        links: Links {
            self_link: Link {
                href: "/api/runs".to_string(),
                method: "GET".to_string(),
            },
            runs_link: Link {
                href: "/api/runs".to_string(),
                method: "GET".to_string(),
            },
            root_link: Link {
                href: "/".to_string(),
                method: "GET".to_string(),
            },
            create_run_link: Link {
                href: "/api/runs".to_string(),
                method: "POST".to_string(),
            },
            create_turn_link: Link {
                href: "/api/turns".to_string(),
                method: "POST".to_string(),
            },
        },
        data: RunData { runs },
    };
    web::Json(serde_json::to_value(response).unwrap())
}

async fn create_run(
    pool: web::Data<Arc<SqlitePool>>,
    run_data: web::Json<CreateRunRequest>,
) -> impl Responder {
    let start_time = DateTime::parse_from_rfc3339(&run_data.start_time)
        .unwrap()
        .with_timezone(&Utc);
    let end_time = DateTime::parse_from_rfc3339(&run_data.end_time)
        .unwrap()
        .with_timezone(&Utc);

    match db::insert_run(
        &pool,
        &start_time.to_rfc3339(),
        &end_time.to_rfc3339(),
        "M 0 50 C 25 25, 75 75, 100 50",
    ).await {
        Ok(run_id) => HttpResponse::Created().json(run_id),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create run: {}", e)),
    }
}

async fn create_turn(
    pool: web::Data<Arc<SqlitePool>>,
    turn_data: web::Json<CreateTurnRequest>,
) -> impl Responder {
    let timestamp_start = DateTime::parse_from_rfc3339(&turn_data.timestamp_start)
        .unwrap()
        .with_timezone(&Utc);
    let timestamp_end = DateTime::parse_from_rfc3339(&turn_data.timestamp_end)
        .unwrap()
        .with_timezone(&Utc);

    match db::insert_turn(
        &pool,
        turn_data.run_id,
        &turn_data.direction,
        turn_data.parallelness,
        turn_data.closeness,
        &turn_data.smoothness,
        timestamp_start,
        timestamp_end,
    )
    .await
    {
        Ok(turn_id) => HttpResponse::Created().json(turn_id),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create turn: {}", e)),
    }
}

async fn turns_get() -> impl Responder {
    HttpResponse::MethodNotAllowed().json(serde_json::json!({
        "error": "Method Not Allowed",
        "message": "This endpoint only accepts POST requests",
        "allowed_methods": ["POST"]
    }))
}

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};

    use super::*;

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().route("/", web::to(index))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
