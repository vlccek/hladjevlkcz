use async_graphql::InputType;
use rocket::http::hyper::body::HttpBody;
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityOrSelect, EntityTrait, InsertResult, QueryFilter};
use crate::orm;
use orm::users::Entity as Users;
use orm::users;

use rocket::response::status;
use rocket::response::status::Unauthorized;
use rocket::yansi::Color::Default;
use sea_orm::sea_query::Token;

use crate::jwt::generate_token;
use crate::orm::sea_orm_active_enums::Userroleenum;
use crate::orm::users::{ActiveModel, Model};
use rocket_auth::{Users as Users_auth, Error, Auth, Signup, Login};


#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct UserLogin {
    id: i64,
    login: String,
    password: String,
}

#[post("/login", format = "json", data = "<userdata>")]
pub async fn login_user(dbc: &State<DatabaseConnection>, userdata: Json<UserLogin>) -> (Status, String) {
    let db = dbc as &DatabaseConnection;

    let mut select = Users::find().select()
        .filter(users::Column::Login.eq(&userdata.login))
        .filter(users::Column::Password.eq(&userdata.password))
        .one(db)
        .await
        .unwrap();

    if select == None {
        select = Users::find()
            .filter(users::Column::Email.eq(&userdata.  login))
            .filter(users::Column::Password.eq(&userdata.password))
            .one(db)
            .await
            .unwrap();
    }


    return match select {
        None => (Status::Unauthorized, "Not valid creditails".parse().unwrap()),
        _ => {
            (Status::Ok, generate_token(select.unwrap().login.unwrap()))
        }
    };
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct UserRegister {
    login: Option<String>,
    password: String,
    email: String,
}


#[post("/register", format = "json", data = "<userdata>")]
pub async fn register_user(dbc: &State<DatabaseConnection>, userdata: Json<UserRegister>) -> Status {
    let db = dbc as &DatabaseConnection;

    let new_user = users::ActiveModel{
        id: ActiveValue::NotSet,
        login : ActiveValue::Set( userdata.login.to_owned()),
        email : ActiveValue::Set(userdata.email.to_owned()),
        password : ActiveValue::Set(userdata.password.to_owned()),
        role: ActiveValue::Set(Option::from(Userroleenum::User)),
        blocked: ActiveValue::Set(Some(false))
    };

    let e = Users::insert(new_user).exec(db).await;

    return match e {
        Err(e) => {println!("{}", e);
        Status::Conflict
        },
        _ => {
            return Status::Ok
            
        }
    };
}
