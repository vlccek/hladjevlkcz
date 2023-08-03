use sea_orm::*;
use rocket::State;
use rocket::serde::json::Json;
use serde_json;
use serde_json::json;


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
async fn test(dbc: &State<DatabaseConnection>) -> Json<Vec<foods::Model>> {
    let db = dbc as &DatabaseConnection;
    let f = Foods::find()
        .all(db )
        .await
        .unwrap() ;

    return Json(f);
}

#[launch]
async fn rocket() -> _ {
    let dbc = match Database::connect("postgres://hello_flask:hello_flask@localhost:5432/hello_flask_dev").await{
        Ok(connection) => connection,
        Err(e) => {
            panic!("Nelze se připojit k db ")
        }
    };
    rocket::build()
        .manage(dbc)
        .mount("/", routes![index, test])
}