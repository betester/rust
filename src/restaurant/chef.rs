use std::thread;

use super::{food::Food, food_status::FoodStatus, food_type::FoodType};
#[derive(Debug)]
pub struct Chef {
    pub chef_id : u32,
    pub food_specialties: Vec<FoodType>,
    pub max_order_taken: u32,
    pub order_taken: u32,
}

impl Chef {
    pub fn cook_food<'a>(&'a self, food: &'a mut Food) -> Result<&mut Food, String> {
        if !(self.is_able_to_cook(&food)) {
            return Err("Cannot cook food".to_string());
        }
        thread::sleep(food.cooking_time_estimation);
        food.change_food_status(FoodStatus::COOKED);
        return Ok(food);
    }

    fn is_able_to_cook(&self, food: &Food) -> bool {
        self.food_specialties.contains(&food.food_type) && self.order_taken <= self.max_order_taken
    }
}

impl PartialEq for Chef {
    fn eq(&self, other: &Self) -> bool {
        self.chef_id == other.chef_id
    }
}