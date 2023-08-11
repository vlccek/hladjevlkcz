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
use sqlx::types::Decimal;

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
    pub struct SelectResult {
        name: String,
        name_en: String,
        price_student: i32,
        count_of_rev: i64,
        avg: Decimal,
    }

    #[derive(FromQueryResult, Serialize)]
    pub struct SelectResultOut {
        name: String,
        name_en: String,
        price_student: i32,
        count_of_rev: i64,
        avg: f64,
    }

    // FIXME: Use correct function once https://github.com/SeaQL/sea-query/pull/671 is resolved
    struct RoundFunction;
    impl Iden for RoundFunction {
        fn unquoted(&self, s: &mut dyn sea_query::Write) {
            write!(s, "ROUND").unwrap();
        }
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
        .filter(current_foods::Column::CanteenId.eq(id))
        .expr_as(Func::avg(Expr::col((ratings::Ratings::Table, ratings::Column::Points))), "avg").to_owned()
        ;


    return match select.into_model::<SelectResult>()
        .all(db)
        .await {
        Err(e) => Json(format!("'err' : '{}'", e).to_owned()).to_string(),
        Ok(s) => {
            /*            let mut out: Vec<SelectResultOut>;
                        for i in s
                        {
                            out.push(
                                SelectResultOut {
                                    name: i.name,
                                    name_en: i.name_en,
                                    avg: i.avg.round_dp(2).to_string(),
                                    price_student: i.price_student,
                                    count_of_rev: i.count_of_rev,


                                }
                            )
                        }*/
            serde_json::to_string(&s).unwrap()
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
            userinout::login_user,
            userinout::register_user
        ])
}