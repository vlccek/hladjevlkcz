use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityOrSelect, EntityTrait, QueryFilter};
use crate::orm;
use orm::users::Entity as Users;
use orm::users;

use rocket::response::status;
use sea_orm::sea_query::Token;

use crate::jwt::generate_token;
use crate::orm::users::Model;


#[derive(Debug, PartialEq, Eq, Deserialize)]
struct User {
    id: i64,
    login: String,
    password: String,
}

#[post("/login", format = "json", data = "<userdata>")]
pub(crate) async fn login_user(dbc: &State<DatabaseConnection>, userdata: Json<User>) -> (status, String) {
    let db = dbc as &DatabaseConnection;

    let mut select = Users::find().select()
        .filter(users::Column::Login.eq(&userdata.login))
        .filter(users::Column::Password.eq(&userdata.password))
        .one(db)
        .await
        .unwrap();

    if select == None {
        select = Users::find()
            .filter(users::Column::Email.eq(&userdata.login))
            .filter(users::Column::Password.eq(&userdata.password))
            .one(db)
            .await
            .unwrap();
    }



    return  match select {
        None => (status::Unauthorized(Some("Not ok")), "Not valid creditails".parse().unwrap()),
        _ => {(status::Accepted(Some("ok")), generate_token(select.unwrap().login.unwrap()));}
    };
}
