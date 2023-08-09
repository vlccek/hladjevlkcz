use std::time::Duration;
use async_graphql::Json;

use rocket::tokio;
use sea_orm::DatabaseConnection;

use reqwest::*;

const SKM_API_URL: &str = "https://www.skm.vutbr.cz/app06/export/mx_json.aspx";

pub async fn download_foods() {
    println!("HI");
    let body = reqwest::get(SKM_API_URL)
        .await
        .unwrap()
        .text()
        .await;

    println!("{}", Json(body.unwrap()).to_string() );
}
