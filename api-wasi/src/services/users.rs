use super::{CreateResponse, ListAllResponse};
use crate::repo::Repo;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Result
};
use log::info;
use validator::Validate;

pub struct UserResponse {
    pub name: String,
    pub email: String,
    pub number: String,
}

#[derive(serde::Deserialize, Default, Validate)]
pub(super) struct CreateUserRequest {
    name: String,
    email: String,
    number: String,
}

pub(super) async fn create(
    State(repo): State<Repo>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<CreateResponse>, StatusCode> {
    if let Err(_) = payload.validate() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    let res = sqlx::query_as!(
        IdResponse,
        "INSERT INTO users (username, email, number)
            VALUES ($1, $2,  $3)
            RETURNING id;",
        name,
        email,
        number,
    )
    .fetch_one(&repo)
    .await
    .unwrap();

    Ok(Json(res))
}

pub async fn list_all_users(
    State(repo): State<Repo>,
) -> Result<Json<ListAllResponse>, StatusCode> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT id, last_name, phone_number, location
        FROM person;
        "#
    )
    .fetch_all(&repo)
    .await
    .unwrap();

    info!("count: {} persons", users.len());
    Ok(Json(users))
}
