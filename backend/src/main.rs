use axum::Json;
use axum::extract::State;
use axum::{
    Router,
    routing::{get, post},
};
use dotenvy::dotenv;
use serde::Serialize;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
#[derive(Serialize)]
struct HealthResponse {
    status: String,
    service: String,
}

#[derive(serde::Serialize, sqlx::FromRow)]
struct Exam {
    id: i32,
    title: String,
    duration_minutes: i32,
}
#[derive(serde::Deserialize)]
struct CreateExamRequest {
    title: String,
    duration_minutes: i32,
}
#[derive(serde::Serialize, sqlx::FromRow)]
struct ActivityLog {
    id: i32,
    student_id: i32,
    exam_id: i32,
    event_type: String,
}
#[derive(serde::Deserialize)]
struct ActivityLogRequest {
    student_id: i32,
    exam_id: i32,
    event_type: String,
}
#[derive(serde::Deserialize)]
struct SubmissionRequest {
    student_id: i32,
    exam_id: i32,
    score: i32,
}

async fn root() -> Json<HealthResponse> {
    let response = HealthResponse {
        status: "OK".to_string(),
        service: "IntegrityX Backend".to_string(),
    };
    Json(response)
}

async fn get_exams(State(pool): State<Arc<PgPool>>) -> Json<Vec<Exam>> {
    let exams = sqlx::query_as::<_, Exam>("SELECT id, title, duration_minutes FROM exams")
        .fetch_all(&*pool)
        .await
        .expect("Failed to Fetch Exams");
    Json(exams)
}
async fn create_exam(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateExamRequest>,
) -> Json<&'static str> {
    sqlx::query("INSERT INTO exams (title, duration_minutes) VALUES ($1,$2)")
        .bind(payload.title)
        .bind(payload.duration_minutes)
        .execute(&*pool)
        .await
        .expect("Failed to create exam");

    Json("exam created")
}
async fn get_logs(
    State(pool): State<Arc<PgPool>>,
) -> Json<Vec<ActivityLog>> {

    let logs = sqlx::query_as::<_,ActivityLog>(
        "SELECT * FROM activity_logs"
    )
    .fetch_all(&*pool)
    .await
    .unwrap();

    Json(logs)
}
async fn log_activity(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<ActivityLogRequest>,
) -> Json<&'static str> {
    sqlx::query("INSERT INTO activity_logs (student_id, exam_id, event_type) VALUES ($1, $2, $3)")
        .bind(payload.student_id)
        .bind(payload.exam_id)
        .bind(payload.event_type)
        .execute(&*pool)
        .await
        .expect("Failed to insert activity log");

    Json("activity logged")
}
async fn get_activity_logs(State(pool): State<Arc<PgPool>>) -> Json<Vec<ActivityLog>> {
    let logs = sqlx::query_as::<_, ActivityLog>(
        "SELECT id, student_id, exam_id, event_type FROM activity_logs",
    )
    .fetch_all(&*pool)
    .await
    .expect("Failed to fetch activity logs");

    Json(logs)
}
async fn submit_exam(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<SubmissionRequest>,
) -> Json<&'static str> {

    sqlx::query(
        "INSERT INTO submissions (student_id, exam_id, score) VALUES ($1,$2,$3)"
    )
    .bind(payload.student_id)
    .bind(payload.exam_id)
    .bind(payload.score)
    .execute(&*pool)
    .await
    .expect("Failed to submit exam");

    Json("submission recorded")
}

#[tokio::main]
async fn main() {
    use std::sync::Arc;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connec to the Database");

    let pool = Arc::new(pool);
    let cors = CorsLayer::permissive();
    let app = Router::new()
        .route("/", get(root))
        .route("/api/activity", get(get_activity_logs).post(log_activity))
        .route("/api/exams", get(get_exams).post(create_exam))
        .route("/api/submissions", post(submit_exam))
        .route("/api/logs", get(get_logs))
        .layer(cors)
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server is running on {}", addr);

    let listner = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listner, app).await.unwrap();
}
