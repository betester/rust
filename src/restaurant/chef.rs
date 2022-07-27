use std::{thread, time};

use super::{food_type::FoodType, food::Food, food_status::FoodStatus};

pub struct Chef {
    food_specialties : Vec<FoodType>,
    max_order_taken : u32,
}

impl Chef {
    pub fn cook_food(&self, mut food: Food) {
        thread::sleep(food.cooking_time_estimation);
        food.change_food_status(FoodStatus::COOKED);
    }
}