use super::food_type::FoodType;

pub struct Food {
    name : String,
    price : f32,
    cooking_time_estimation : f32,
    eating_time_estimation : f32,
    food_type : FoodType,
}