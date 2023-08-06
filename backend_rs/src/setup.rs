// src/setup.rs

use sea_orm::*;

// Replace with your database URL and database name
const DATABASE_URL: &str = "postgres://hello_flask:hello_flask@localhost:5432/hello_flask_dev";
const DB_NAME: &str = "hello_flask_dev";

pub(super) async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;


    Ok(db)
}

