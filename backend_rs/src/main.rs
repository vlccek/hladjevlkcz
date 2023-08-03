use sea_orm::*;
use rocket::State;
use rocket::serde::json::Json;
use sea_orm::prelude::Json;
use rocket::http::Status;


mod orm;
use orm::foods::Entity as Foods;
use orm::foods;
use crate::orm::foods::Model;


// import all orm entities

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
async fn test(dbc: &State<DatabaseConnection>) ->  Result<Json<Vec<foods::Model>>, Status>{
    Ok(Json(
        Foods::find()
        .filter(foods::Column::Name.contains("knedlík"))
        .all(&dbc)
        .await
        .unwrap()
    ))
}

#[launch]
async fn rocket() -> _ {
    let dbc = match Database::connect("postgresql://hello_flask:hello_flask@db:5432/hello_flask_dev").await{
        Ok(connection) => connection,
        Err(e) => {
            panic!("Nelze se připojit k db ")
        }
    };
    rocket::build()
        .manage(dbc)
        .mount("/", routes![index])
}