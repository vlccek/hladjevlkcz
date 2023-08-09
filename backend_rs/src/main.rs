use std::borrow::{BorrowMut, Borrow};

use orm::sea_orm_active_enums::Foodtypeenum;
use sea_orm::*;
use rocket::State;
use rocket::serde::json::Json;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};


mod orm;
use orm::foods::{Entity as Foods};
use orm::canteens::{Entity as Canteen, Model};
use orm::{canteens as ccanteens, current_foods::Entity as cur_food};
use orm::{foods, current_foods};


pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

// import all orm entities

#[macro_use]
extern crate rocket;

#[get("/test")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
async fn test(dbc: &State<DatabaseConnection>) -> Json<Vec<foods::Model>> {
    let db = dbc as &DatabaseConnection;
    let f = Foods::find()
        .all(db)
        .await
        .unwrap();
    return Json(f);
}
#[get("/sides")]
async fn sides(dbc: &State<DatabaseConnection>) -> Json<Vec<foods::Model>> {
    let db = dbc as &DatabaseConnection;
    let foods: Vec<foods::Model> = Foods::find()
        .filter(foods::Column::FoodType.eq(Foodtypeenum::Side))
        .all(db)
        .await
        .unwrap();
    
    return Json(foods);
}

#[get("/canteen_menu/<id>")]
async fn canteens(dbc: &State<DatabaseConnection>, id: u8) -> Json<Vec<current_foods::Model>> {
    let db = dbc as &DatabaseConnection;
    let canteen = Canteen::find_by_id(id).one(db).await.unwrap().unwrap();
    let foods: Vec<current_foods::Model> = canteen.find_related(cur_food)
        .filter(ccanteens::Column::Name.contains("hello"))
        .all(db)
        .await
        .unwrap();
    
    return Json(foods);
}

#[launch]
async fn rocket() -> _ {
    let dbc = match Database::connect("postgres://hello_flask:hello_flask@localhost:5432/hello_flask_dev").await{
        Ok(connection) => connection,
        Err(_) => {
            panic!("Nelze se připojit k db.")
        }
    };
    rocket::build()
        .manage(dbc)
        .mount("/api", routes![index, test, sides, canteens]).attach(CORS)
}
