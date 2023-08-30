// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "foodcategoryenum"))]
    pub struct Foodcategoryenum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "foodtypeenum"))]
    pub struct Foodtypeenum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "userroleenum"))]
    pub struct Userroleenum;
}

diesel::table! {
    allergens (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 200]
        descr -> Nullable<Varchar>,
    }
}

diesel::table! {
    canteens (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        address -> Nullable<Varchar>,
        opened_first -> Nullable<Array<Nullable<Time>>>,
        opened_second -> Nullable<Array<Nullable<Time>>>,
        opened_first_friday -> Nullable<Array<Nullable<Time>>>,
        opened_second_friday -> Nullable<Array<Nullable<Time>>>,
    }
}

diesel::table! {
    current_foods (id) {
        id -> Int4,
        food_id -> Nullable<Int4>,
        canteen_id -> Nullable<Int4>,
        last_available -> Nullable<Date>,
        available -> Nullable<Bool>,
    }
}

diesel::table! {
    food_allergens (food_id, allergen_id) {
        food_id -> Int4,
        allergen_id -> Int4,
    }
}

diesel::table! {
    food_ingredients (food_id, ingredient_id) {
        food_id -> Int4,
        ingredient_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Foodtypeenum;
    use super::sql_types::Foodcategoryenum;

    foods (id) {
        id -> Int4,
        #[max_length = 300]
        name -> Varchar,
        #[max_length = 300]
        name_en -> Varchar,
        food_type -> Nullable<Foodtypeenum>,
        category -> Nullable<Array<Nullable<Foodcategoryenum>>>,
        #[max_length = 50]
        weight -> Nullable<Varchar>,
        price_student -> Int4,
        price_employee -> Int4,
        price_extern -> Int4,
    }
}

diesel::table! {
    ingredients (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        is_vegan -> Nullable<Bool>,
        is_vegetarian -> Nullable<Bool>,
        is_gluten_free -> Nullable<Bool>,
        is_checked -> Nullable<Bool>,
    }
}

diesel::table! {
    ratings (id) {
        id -> Int4,
        points -> Int4,
        #[max_length = 300]
        text -> Nullable<Varchar>,
        added -> Nullable<Timestamp>,
        user_id -> Nullable<Int4>,
        food_id -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Userroleenum;

    users (id) {
        id -> Int4,
        #[max_length = 100]
        login -> Nullable<Varchar>,
        #[max_length = 250]
        password -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        blocked -> Nullable<Bool>,
        role -> Nullable<Userroleenum>,
    }
}

diesel::joinable!(current_foods -> canteens (canteen_id));
diesel::joinable!(current_foods -> foods (food_id));
diesel::joinable!(food_allergens -> allergens (allergen_id));
diesel::joinable!(food_allergens -> foods (food_id));
diesel::joinable!(food_ingredients -> foods (food_id));
diesel::joinable!(food_ingredients -> ingredients (ingredient_id));
diesel::joinable!(ratings -> current_foods (food_id));
diesel::joinable!(ratings -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    allergens,
    canteens,
    current_foods,
    food_allergens,
    food_ingredients,
    foods,
    ingredients,
    ratings,
    users,
);
