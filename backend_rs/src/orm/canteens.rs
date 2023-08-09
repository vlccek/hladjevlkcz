//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "canteens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
    pub opened_first: Option<Vec<Time>>,
    pub opened_second: Option<Vec<Time>>,
    pub opened_first_friday: Option<Vec<Time>>,
    pub opened_second_friday: Option<Vec<Time>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::current_foods::Entity")]
    CurrentFoods,
}

impl Related<super::current_foods::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CurrentFoods.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
