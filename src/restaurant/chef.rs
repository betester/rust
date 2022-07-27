use std::thread;

use super::{food::Food, food_status::FoodStatus, food_type::FoodType, order::Order};

pub struct Chef<'a> {
    food_specialties: Vec<FoodType>,
    max_order_taken: u32,
    order_taken: u32,
}

impl<'a> Chef<'a> {
    // TODO: make asynchronous on the future
    pub fn handle_order(&mut self, order: &mut Order) {
        self.order_taken += 1;
        for food in &mut order.ordered_food {
            match self.cook_food(food) {
                Ok(i) => {
                    order.notify_customer(food);
                }
                Err(err_msg) => println!("{}",err_msg),
            }
        }
        self.order_taken -= 1;
    }

    pub fn cook_food(&mut self, food: &'a mut Food) -> Result<&mut Food, String> {
        if !(self.is_able_to_cook(&food)) {
            return Err("Cannot cook food".to_string());
        }
        thread::sleep(food.cooking_time_estimation);
        food.change_food_status(FoodStatus::COOKED);
        return Ok(&mut food);
    }

    fn is_able_to_cook(&self, food: &Food) -> bool {
        self.food_specialties.contains(&food.food_type) && self.order_taken <= self.max_order_taken
    }
}
