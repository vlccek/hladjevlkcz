//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "foodcategoryenum")]
pub enum Foodcategoryenum {
    #[sea_orm(string_value = "gluten")]
    Gluten,
    #[sea_orm(string_value = "vegan")]
    Vegan,
    #[sea_orm(string_value = "vegetarian")]
    Vegetarian,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "foodtypeenum")]
pub enum Foodtypeenum {
    #[sea_orm(string_value = "desert")]
    Desert,
    #[sea_orm(string_value = "main")]
    Main,
    #[sea_orm(string_value = "salad")]
    Salad,
    #[sea_orm(string_value = "side")]
    Side,
    #[sea_orm(string_value = "soup")]
    Soup,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "userroleenum")]
pub enum Userroleenum {
    #[sea_orm(string_value = "admin")]
    Admin,
    #[sea_orm(string_value = "mod")]
    Mod,
    #[sea_orm(string_value = "user")]
    User,
}
