//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "current_foods")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub food_id: Option<i32>,
    pub canteen_id: Option<i32>,
    pub last_available: Option<TimeDate>,
    pub available: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::canteens::Entity",
        from = "Column::CanteenId",
        to = "super::canteens::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Canteens,
    #[sea_orm(
        belongs_to = "super::foods::Entity",
        from = "Column::FoodId",
        to = "super::foods::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Foods,
    #[sea_orm(has_many = "super::ratings::Entity")]
    Ratings,
}

impl Related<super::canteens::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Canteens.def()
    }
}

impl Related<super::foods::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Foods.def()
    }
}

impl Related<super::ratings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ratings.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::canteens::Entity")]
    Canteens,
    #[sea_orm(entity = "super::foods::Entity")]
    Foods,
    #[sea_orm(entity = "super::ratings::Entity")]
    Ratings,
}
