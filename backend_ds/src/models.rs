// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use diesel::prelude::*;
use chrono;
use serde::Serialize;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use crate::schema::*;
use diesel::connection::SimpleConnection;
use diesel::deserialize::{self, FromSql};
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::*;
use std::io::Write;
use crate::enumtype::{Foodcategoryenum, Foodtypeenum, Userroleenum};


#[derive(Queryable, Debug, Identifiable)]
pub struct Allergen {
    pub id: i32,
    pub name: Option<String>,
    pub descr: Option<String>,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct Canteen {
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
    pub opened_first: Option<Vec<Option<NaiveTime>>>,
    pub opened_second: Option<Vec<Option<NaiveTime>>>,
    pub opened_first_friday: Option<Vec<Option<NaiveTime>>>,
    pub opened_second_friday: Option<Vec<Option<NaiveTime>>>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize)]
#[diesel(belongs_to(Food))]
#[diesel(table_name = current_foods)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CurrentFood {
    pub id: i32,
    pub food_id: Option<i32>,
    pub canteen_id: Option<i32>,
    pub last_available: Option<NaiveDate>,
    pub available: Option<bool>,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(food_id, allergen_id))]
pub struct FoodAllergen {
    pub food_id: i32,
    pub allergen_id: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(food_id, ingredient_id))]
pub struct FoodIngredient {
    pub food_id: i32,
    pub ingredient_id: i32,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Serialize)]
#[diesel(table_name = foods)]
pub struct Food {
    pub id: i32,
    pub name: String,
    pub name_en: String,
    pub food_type: Option<Foodtypeenum>,
    pub category: Option<Vec<Option<Foodcategoryenum>>>,
    pub weight: Option<String>,
    pub price_student: i32,
    pub price_employee: i32,
    pub price_extern: i32,
}

#[derive(Queryable, Debug)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub is_vegan: Option<bool>,
    pub is_vegetarian: Option<bool>,
    pub is_gluten_free: Option<bool>,
    pub is_checked: Option<bool>,
}

pub struct Rating {
    pub id: i32,
    pub points: i32,
    pub text: Option<String>,
    pub added: Option<NaiveDateTime>,
    pub user_id: Option<i32>,
    pub food_id: Option<i32>,
}

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub login: Option<String>,
    pub password: String,
    pub email: String,
    pub blocked: Option<bool>,
    pub role: Option<Userroleenum>,
}
