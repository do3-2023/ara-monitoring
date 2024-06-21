mod users;
use crate::repo::Repo;

use axum::{
    extract::Host,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use users::UserResponse;

pub async fn router() -> Router<()> {
    let repo = Repo::new().await.unwrap();

    Router::new()
        .route("/", get(default_page))
        .route("/health", get(health))
        .route("/users", get(users::list_all_users))
        .route("/user", post(users::create))
        .with_state(repo)
}

async fn default_page() -> impl IntoResponse {
    let resp = StatusBody::new("default".to_string());

    (StatusCode::OK, Json(resp))
}

async fn health(Host(host): Host) -> Result<Json<HealthBody>, StatusCode> {
    log::debug!("health check from: '{}'", host);

    let res = HealthBody {
        status: "ok".to_string(),
    };

    Ok(Json(res))
}

#[derive(serde::Serialize)]
struct CreateResponse {
    id: uuid::Uuid,
}

struct ListAllResponse {
    users: Vec<UserResponse>,
}

#[derive(serde::Serialize)]
struct StatusBody {
    status: String,
}

impl StatusBody {
    fn new(status: String) -> Self {
        Self { status }
    }
}

#[derive(serde::Serialize)]
struct HealthBody {
    status: String,
}
