use async_graphql::InputType;
use rocket::http::hyper::body::HttpBody;
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityOrSelect, EntityTrait, InsertResult, QueryFilter, Condition};
use crate::orm;
use orm::users::Entity as Users;
use orm::users;

use crate::orm::sea_orm_active_enums::Userroleenum;

use rocket_jwt::jwt;

use password_auth::{generate_hash, verify_password};

use scrypt::Scrypt;

static SECRET_KEY: &str = "secret_key";

#[jwt(SECRET_KEY, exp = 10, leeway = 10)]
pub struct UserClaim {
    id: i32,
}


#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct UserLogin {
    login: String,
    password: String,
}

#[post("/login", format = "json", data = "<userdata>")]
pub async fn login_user(dbc: &State<DatabaseConnection>, userdata: Json<UserLogin>) -> (Status, String) {
    let db = dbc as &DatabaseConnection;

    let select = Users::find().select()
        .filter(
            Condition::all().add(Condition::any()
                .add(users::Column::Login.eq(&userdata.login))
                .add(users::Column::Email.eq(&userdata.login))
            )
        )
        .one(db)
        .await
        .unwrap();

    let data = match select {
        Some(a) => {
            a
        }
        _ => {
            return (Status::Unauthorized, "Not valid creditails".parse().unwrap());
        }
    };

    match verify_password(userdata.password.to_owned(), data.password.as_str()) {
        Ok(..) => {}
        _ => return (Status::Unauthorized, "Not valid creditails".parse().unwrap()),
    };


    let user_claim = UserClaim {
        id: data.id,
    };

    let token = UserClaim::sign(user_claim);

    (Status::Ok, token)
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

    let hash = generate_hash(userdata.password.to_owned());

    let new_user = users::ActiveModel {
        id: ActiveValue::NotSet,
        login: ActiveValue::Set(userdata.login.to_owned()),
        email: ActiveValue::Set(userdata.email.to_owned()),
        password: ActiveValue::Set(hash),
        role: ActiveValue::Set(Option::from(Userroleenum::User)),
        blocked: ActiveValue::Set(Some(false)),
    };

    let e = Users::insert(new_user).exec(db).await;

    return match e {
        Err(e) => {
            println!("{}", e);
            Status::Conflict
        }
        _ => {
            return Status::Ok;
        }
    };
}
