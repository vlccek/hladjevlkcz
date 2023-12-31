//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use super::sea_orm_active_enums::Foodcategoryenum;
use super::sea_orm_active_enums::Foodtypeenum;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "foods")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
    #[sea_orm(unique)]
    pub name_en: String,
    pub food_type: Option<Foodtypeenum>,
    pub category: Option<Vec<Foodcategoryenum>>,
    pub weight: Option<String>,
    pub price_student: i32,
    pub price_employee: i32,
    pub price_extern: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::current_foods::Entity")]
    CurrentFoods,
    #[sea_orm(has_many = "super::food_allergens::Entity")]
    FoodAllergens,
    #[sea_orm(has_many = "super::food_ingredients::Entity")]
    FoodIngredients,
}

impl Related<super::current_foods::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CurrentFoods.def()
    }
}

impl Related<super::food_allergens::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FoodAllergens.def()
    }
}

impl Related<super::food_ingredients::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FoodIngredients.def()
    }
}

impl Related<super::allergens::Entity> for Entity {
    fn to() -> RelationDef {
        super::food_allergens::Relation::Allergens.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::food_allergens::Relation::Foods.def().rev())
    }
}

impl Related<super::ingredients::Entity> for Entity {
    fn to() -> RelationDef {
        super::food_ingredients::Relation::Ingredients.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::food_ingredients::Relation::Foods.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::current_foods::Entity")]
    CurrentFoods,
    #[sea_orm(entity = "super::food_allergens::Entity")]
    FoodAllergens,
    #[sea_orm(entity = "super::food_ingredients::Entity")]
    FoodIngredients,
    #[sea_orm(entity = "super::allergens::Entity")]
    Allergens,
    #[sea_orm(entity = "super::ingredients::Entity")]
    Ingredients,
}
