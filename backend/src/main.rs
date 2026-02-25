use axum::{routing::get, Router};
use std::net::SocketAddr;
use axum::Json;
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;
use serde::Serialize;
use axum::extract::State;
use std::sync::Arc;
use sqlx::PgPool;
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

async fn root() -> Json<HealthResponse> {
    let response = HealthResponse {
        status: "OK".to_string(),
        service: "IntegrityX Backend".to_string(),
    };
    Json(response)
}

async fn get_exams(
    State(pool): State<Arc<PgPool>>,
)-> Json<Vec<Exam>>{
    let exams = sqlx::query_as::<_,Exam>(
        "SELECT id, title, duration_minutes FROM exams"
    )
    .fetch_all(&*pool)
    .await
    .expect("Failed to Fetch Exams");
Json(exams)
}

#[tokio::main]
async fn main(){
    use std::sync::Arc;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to connec to the Database");
    
    let pool = Arc::new(pool); 
    
    let app = Router::new()
        .route("/",get(root))
        .route("/api/exams",get(get_exams))
        .with_state(pool);

    let addr = SocketAddr::from(([127,0,0,1],3000));
    println!("Server is running on {}",addr);

    let listner = tokio::net::TcpListener::bind(addr)
    .await
    .unwrap();
    axum::serve(listner, app)
    .await
    .unwrap();
}