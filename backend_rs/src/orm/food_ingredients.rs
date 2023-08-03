//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "food_ingredients")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub food_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub ingredient_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::foods::Entity",
        from = "Column::FoodId",
        to = "super::foods::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Foods,
    #[sea_orm(
        belongs_to = "super::ingredients::Entity",
        from = "Column::IngredientId",
        to = "super::ingredients::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Ingredients,
}

impl Related<super::foods::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Foods.def()
    }
}

impl Related<super::ingredients::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ingredients.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
