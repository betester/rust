use std::thread;

use super::{food::Food, food_status::FoodStatus, food_type::FoodType};

pub struct Chef {
    food_specialties: Vec<FoodType>,
    max_order_taken: u32,
    order_taken: u32,
}

impl Chef {
    // TODO: make asynchronous on the future
    pub fn handle_order() {
        
    }

    pub fn cook_food(&mut self, mut food: Food) {
        if !(self.is_able_to_cook(&food)) {
            return;
        }
        thread::sleep(food.cooking_time_estimation);
        food.change_food_status(FoodStatus::COOKED);
    }

    fn is_able_to_cook(&self, food: &Food) -> bool {
        self.food_specialties.contains(&food.food_type) && self.order_taken <= self.max_order_taken
    }
}
