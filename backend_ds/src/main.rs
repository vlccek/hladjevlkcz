mod enumtype;
mod models;
mod schema;
mod setup;

use std::ffi::c_uint;
use diesel::{PgConnection, prelude, QueryDsl, ExpressionMethods, RunQueryDsl, select};
use diesel::associations::HasTable;
use rocket::Data;
use enumtype::Userroleenum;
use schema::{users};
use rocket::serde::json::Json;
use self::models::*;
use rocket_sync_db_pools::{database, diesel};
use crate::schema::ratings::dsl::ratings;
use diesel::SelectableHelper;

use crate::schema::foods::dsl::foods;
use crate::schema::current_foods::dsl::*;
use self::schema::users::dsl::*;

use crate::models::*;
use diesel::prelude::*;

#[macro_use]
extern crate rocket;

#[database("my_db")]
struct Database(diesel::PgConnection);

#[get("/")]
async fn index(conn: Database) -> Json<models::User> {
    let resulst = conn.run(|c| {
        schema::users::table
            .first::<User>(c)
    }).await.unwrap();
    Json(resulst)

    // .filter(role.eq(Userroleenum::User))
}

async fn food_by_cateen(conn: Database)-> Json<foods> {
    let resulst = conn.run(|c| {
        schema::foods::table
            .first::<foods>(c)
    }).await.unwrap();
    Json(resulst)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Database::fairing())
        .mount("/", routes![index])
}

