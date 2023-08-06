use sea_orm::*;
use rocket::State;
use rocket::serde::json::Json;
use sea_orm::sea_query::{Func, ValueType};
use sea_orm::sea_query::OnConflictUpdate::Expr;

mod setup;

use setup::set_up_db;


mod orm;

use orm::foods::Entity as Foods;
use orm::foods;

use orm::current_foods::Entity as Current_foods;
use orm::current_foods;

use orm::ratings::Entity as Ratings;
use crate::orm::ratings;


// import all orm entities


use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_rocket::*;
use rocket::{response::content, *};
use rocket::serde::Serialize;
use schema::*;


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
        .all(db)
        .await
        .unwrap();
    return Json(f);
}


#[get("/canteen/<id>")]
async fn canteen_foods(dbc: &State<DatabaseConnection>, id: i32) -> String {
    #[derive(FromQueryResult, Serialize)]
    struct SelectResult {
        name: String,
        name_en: String,
        price_student: i32,
    }

    let db = dbc as &DatabaseConnection;
    let select = Current_foods::find()
        .join(JoinType::Join, current_foods::Relation::Foods.def())
        .join(JoinType::Join, current_foods::Relation::Ratings.def())
        .columns([foods::Column::Name, foods::Column::NameEn, foods::Column::PriceStudent])
        .filter(current_foods::Column::CanteenId.eq(id));


    let f = match select.into_model::<SelectResult>()
        .all(db)
        .await {
        Some(x) => return serde_json::to_string(&x).unwrap(),
        None => return "{err: Nelze neni ve smlouve}".to_owned(),
    };
}


#[launch]
async fn rocket() -> _ {
    let dbc = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };


    rocket::build()
        .manage(dbc)
        .mount("/", routes![index, canteen_foods ])
}