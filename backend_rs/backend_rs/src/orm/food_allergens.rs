//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "food_allergens")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub food_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub allergen_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::allergens::Entity",
        from = "Column::AllergenId",
        to = "super::allergens::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Allergens,
    #[sea_orm(
        belongs_to = "super::foods::Entity",
        from = "Column::FoodId",
        to = "super::foods::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Foods,
}

impl Related<super::allergens::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Allergens.def()
    }
}

impl Related<super::foods::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Foods.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::allergens::Entity")]
    Allergens,
    #[sea_orm(entity = "super::foods::Entity")]
    Foods,
}
