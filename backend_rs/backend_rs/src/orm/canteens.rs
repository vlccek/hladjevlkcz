//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "canteens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
    pub opened_first: Option<Vec<TimeTime>>,
    pub opened_second: Option<Vec<TimeTime>>,
    pub opened_first_friday: Option<Vec<TimeTime>>,
    pub opened_second_friday: Option<Vec<TimeTime>>,
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::current_foods::Entity")]
    CurrentFoods,
}
