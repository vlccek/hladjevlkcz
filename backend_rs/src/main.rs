use chrono::format::Numeric;
use sea_orm::*;
use sea_orm::{Iden};
use rocket::State;
use rocket::serde::json::Json;

mod setup;

use setup::set_up_db;

mod orm;

use orm::foods::Entity as Foods;
use orm::foods;
use orm::current_foods::Entity as Current_foods;
use orm::current_foods;
use orm::ratings::Entity as Ratings;
use crate::orm::ratings;
use orm::canteens::Entity as Canteens;
use orm::canteens;


use rocket::{response::content, *};
use rocket::serde::Serialize;


mod userinout;


use sea_query::{Expr, Func};
use sqlx::types::{Decimal};

use std::time::{Duration, Instant};
use serde_json::json;
use crate::orm::sea_orm_active_enums::{Foodcategoryenum, Foodtypeenum};


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


#[get("/menues/<id>/today")]
async fn canteen_foods(dbc: &State<DatabaseConnection>, id: i32) -> String {
    #[derive(FromQueryResult, Serialize)]
    pub struct SelectResult {
        id: i32,
        name: String,
        name_en: String,
        price_student: i32,
        count_of_rev: i64,
        avg: Decimal,
    }

    #[derive(FromQueryResult, Serialize)]
    pub struct SelectResultOut {
        id: i32,
        name: String,
        name_en: String,
        price_student: i32,
        count_of_rev: i64,
        avg: f64,
    }

    let db = dbc as &DatabaseConnection;
    let select = Current_foods::find()
        .join(JoinType::Join, current_foods::Relation::Foods.def())
        .join(JoinType::Join, current_foods::Relation::Ratings.def())
        .column_as(ratings::Column::Points.count(), "count_of_rev")
        .group_by(current_foods::Column::Id)
        .group_by(foods::Column::Name)
        .group_by(foods::Column::NameEn)
        .group_by(foods::Column::PriceStudent)
        .columns([foods::Column::Name, foods::Column::NameEn, foods::Column::PriceStudent])
        .columns([current_foods::Column::Id])
        .filter(current_foods::Column::CanteenId.eq(id))
        .expr_as(Func::avg(Expr::col((ratings::Ratings::Table, ratings::Column::Points))), "avg").to_owned();


    return match select.into_model::<SelectResult>()
        .all(db)
        .await {
        Err(e) => Json(format!("'err' : '{}'", e).to_owned()).to_string(),
        Ok(s) => {
            let mut out: Vec<SelectResultOut> = Vec::with_capacity(200);
            for i in s
            {
                let nw = SelectResultOut {
                    id: i.id,
                    name: i.name,
                    name_en: i.name_en,
                    avg: i.avg.round_dp(2).to_string().parse().unwrap(),
                    price_student: i.price_student,
                    count_of_rev: i.count_of_rev,

                };
                out.push(nw);
            }
            let json_out = serde_json::json!(out);
            serde_json::to_string(&json_out).unwrap()
        }
    };
}

#[get("/meal/<id>")]
async fn food_detail(dbc: &State<DatabaseConnection>, id: i32) -> String {
    #[derive(FromQueryResult, Serialize)]
    pub struct SelectResultOut {
        id: i32,
        name: String,
        name_en: String,
        price_extern: i32,
        price_employee: i32,
        price_student: i32,
        food_type: Option<Foodtypeenum>,
        category: Option<Vec<Foodcategoryenum>>,
        weight: Option<String>
    }

    let db = dbc as &DatabaseConnection;
    let select = Current_foods::find()
        .join(JoinType::Join, current_foods::Relation::Foods.def())
        .filter(current_foods::Column::CanteenId.eq(id))
        .filter(current_foods::Column::Available.eq(true))
        .columns([foods::Column::Name, foods::Column::NameEn, foods::Column::PriceStudent, foods::Column::FoodType, foods::Column::Weight, foods::Column::Category, foods::Column::PriceEmployee, foods::Column::PriceExtern]);


    return match select.into_model::<SelectResultOut>()
        .all(db)
        .await {
        Err(e) => Json(format!("'err' : '{}'", e).to_owned()).to_string(),
        Ok(s) => {
            let j = serde_json::json!(s);
            serde_json::to_string(&j).unwrap()
        }
    };
}


#[get("/canteens")]
async fn all_canteens(dbc: &State<DatabaseConnection>) -> String {
    let db = dbc as &DatabaseConnection;

    #[derive(FromQueryResult, Serialize)]
    struct SelectResultCanteens {
        id: i32,
        name: String,
    }

    let select = Canteens::find()
        .columns([canteens::Column::Id, canteens::Column::Id]);

    return match select.into_model::<SelectResultCanteens>()
        .all(db)
        .await {
        Err(e) => format!("err : {}", e).to_owned(),
        Ok(s) => serde_json::to_string(&s).unwrap(),
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
        .mount("/api", routes![index,
            canteen_foods,
            all_canteens,
            food_detail,
            userinout::login_user,
            userinout::register_user
        ])
}