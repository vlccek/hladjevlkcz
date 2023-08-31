use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

use serde::Serialize;

#[derive(Debug, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = crate::schema::sql_types::Foodcategoryenum)]
pub enum Foodcategoryenum {
    Gluten,
    Vegan,
    Vegetarian,
}

impl ToSql<crate::schema::sql_types::Foodcategoryenum, Pg> for Foodcategoryenum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Foodcategoryenum::Gluten => out.write_all(b"gluten")?,
            Foodcategoryenum::Vegan => out.write_all(b"vegan")?,
            Foodcategoryenum::Vegetarian => out.write_all(b"vegetarian")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::Foodcategoryenum, Pg> for Foodcategoryenum {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"vegan" => Ok(Foodcategoryenum::Gluten),
            b"vegetarian" => Ok(Foodcategoryenum::Vegetarian),
            b"gluten" => Ok(Foodcategoryenum::Vegan),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}


#[derive(Debug, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = crate::schema::sql_types::Foodtypeenum)]
pub enum Foodtypeenum {
    Desert,
    Main,
    Salad,
    Side,
    Soup,
}

impl ToSql<crate::schema::sql_types::Foodtypeenum, Pg> for Foodtypeenum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Foodtypeenum::Desert => out.write_all(b"desert")?,
            Foodtypeenum::Main => out.write_all(b"main")?,
            Foodtypeenum::Salad => out.write_all(b"salad")?,
            Foodtypeenum::Side => out.write_all(b"side")?,
            Foodtypeenum::Soup => out.write_all(b"soup")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::Foodtypeenum, Pg> for Foodtypeenum {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"desert" => Ok(Foodtypeenum::Desert),
            b"main" => Ok(Foodtypeenum::Main),
            b"salad" => Ok(Foodtypeenum::Salad),
            b"side" => Ok(Foodtypeenum::Side),
            b"soup" => Ok(Foodtypeenum::Soup),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, AsExpression, FromSqlRow, Serialize)]
#[diesel(sql_type = crate::schema::sql_types::Userroleenum)]
pub enum Userroleenum {
    Admin,
    Mod,
    User,
}

impl ToSql<crate::schema::sql_types::Userroleenum, Pg> for Userroleenum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Userroleenum::Admin => out.write_all(b"admin")?,
            Userroleenum::Mod => out.write_all(b"mod")?,
            Userroleenum::User => out.write_all(b"user")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::Userroleenum, Pg> for Userroleenum {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"user" => Ok(Userroleenum::User),
            b"mod" => Ok(Userroleenum::Mod),
            b"admin" => Ok(Userroleenum::Admin),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
