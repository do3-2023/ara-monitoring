#![allow(dead_code)]

use anyhow::Result;
use http::{Request, Response};
use spin_sdk::{
    http_component, pg::{self, Connection, Decode, ParameterValue}, variables
};
use serde::{Deserialize, Serialize};

// The environment variable set in `spin.toml` that points to the
// address of the Pg server that the component will write to
const DB_URL_ENV: &str = "DB_URL";

#[derive(Debug, Clone, Deserialize)]
struct NewUser {
    name: String,
    email: String,
    number: String,
}

#[derive(Debug, Clone, Serialize)]
struct User {
    id: i32,
    name: String,
    email: String,
    number: String,
}

#[derive(Serialize)]
struct JsonResponse {
    count: usize,
    users: Vec<User>,
    column_info: String,
}

impl TryFrom<&pg::Row> for User {
    type Error = anyhow::Error;

    fn try_from(row: &pg::Row) -> Result<Self, Self::Error> {
        let id = i32::decode(&row[0])?;
        let name = String::decode(&row[1])?;
        let email = String::decode(&row[2])?;
        let number = String::decode(&row[3])?;

        Ok(Self {
            id,
            name,
            email,
            number,
        })
    }
}

#[http_component]
fn process(req: Request<()>) -> Result<Response<String>> {
    match req.uri().path() {
        "/users" => read_users(req),
        "/adduser" => add_user(req),
        "/" => check_health(req),
        _ => Ok(http::Response::builder()
            .status(404)
            .body("Not found".into())?),
    }
}

fn read_users(_req: Request<()>) -> Result<Response<String>> {
    let address = variables::get("db_url").expect("db_url not found");
    let conn = Connection::open(&address)?;

    let sql = "SELECT id, name, email, number FROM users";
    let rowset = conn.query(sql, &[])?;

    let column_summary = rowset
        .columns
        .iter()
        .map(format_col)
        .collect::<Vec<_>>()
        .join(", ");

    let mut users = vec![];

    for row in rowset.rows {
        let user = User::try_from(&row)?;
        users.push(user);
    }

    let json_response = JsonResponse {
        count: users.len(),
        users,
        column_info: column_summary,
    };

    let response_body = serde_json::to_string(&json_response)?;
    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(response_body)?)
}

fn add_user(_req: Request<()>) -> Result<Response<String>> {
    let address = std::env::var(DB_URL_ENV)?;
    let conn = pg::Connection::open(&address)?;

    // Read the request body
    // let body_bytes = to_bytes(req.into_body()).wait?;
    // let body_str = std::str::from_utf8(&body_bytes)?;
    // let user = serde_json::from_str::<NewUser>(&body_str)?;
    // let params = vec![ParameterValue::Str(user.name), ParameterValue::Str(user.email), ParameterValue::Str(user.number)];
    
    let sql = "INSERT INTO users (name, email, number) VALUES ('aaa', 'bbb', 'ccc')";
    let nrow_executed = conn.execute(sql, &[])?;

    println!("nrow_executed: {}", nrow_executed);

    let sql = "SELECT COUNT(id) FROM users ";
    let rowset = conn.query(sql, &[])?;
    let row = &rowset.rows[0];
    let count = i64::decode(&row[0])?;
    let response = format!("Count: {}\n", count);

    Ok(http::Response::builder().status(200).body(response)?)
}

fn check_health(_req: Request<()>) -> Result<Response<String>> {
    let address = variables::get("db_url").expect("db_url not found");
    let conn = pg::Connection::open(&address)?;
    let sql = "SELECT pg_backend_pid()";

    let get_pid = || {
        let rowset = conn.query(sql, &[])?;
        let row = &rowset.rows[0];

        i32::decode(&row[0])
    };

    assert_eq!(get_pid()?, get_pid()?);

    let response = format!("pg_backend_pid: {}\n", get_pid()?);

    Ok(http::Response::builder().status(200).body(response)?)
}

fn format_col(column: &pg::Column) -> String {
    format!("{}:{:?}", column.name, column.data_type)
}
