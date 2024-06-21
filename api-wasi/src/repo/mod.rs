use sqlx::postgres::PgPool;

#[derive(Clone)]
pub struct Repo {
    db_name: String,
    db: PgPool,
}

impl Repo {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let host = std::env::var("DB_HOST").unwrap_or("localhost".to_string());
        let port = std::env::var("DB_PORT").unwrap_or("5432".to_string());
        let user = std::env::var("DB_USER").unwrap_or("kubewasm".to_string());
        let password = std::env::var("DB_PASSWORD").unwrap_or("kubewasm".to_string());
        let dbname = std::env::var("DB_NAME").unwrap_or("kubewasm".to_string());

        let db = PgPool::connect(
            format!("postgres://{user}:{password}@{host}:{port}/{dbname}")
                .as_str(),
        )
        .await?;

        log::info!("initialising database...");
        // sqlx::migrate!("./migrations").run(&db).await?;

        Ok(Self {
            db_name: dbname,
            db,
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::Repo;

//     #[tokio::test]
//     async fn initialization() {
//         Repo::new().await.unwrap();
//     }
// }