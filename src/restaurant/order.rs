use std::path::Iter;

use super::{food::Food, food_status::FoodStatus, menu::Menu};
#[derive(Debug)]
pub struct Order {
    // TODO: for efficient implementation, use avl tree
    pub ordered_food: Vec<Food>,
    pub customer: String,
}

impl Order {
    pub fn add_food(&mut self, food_name: &String, menu: &Menu) {
        if !self.is_food_ordered(food_name) {
            match menu.get_food_by_name(food_name) {
                Some(i) => {
                    self.ordered_food.push(i.clone());
                }
                None => println!("There are no such food"),
            }
        }
    }

    pub fn remove_food(&mut self, food: Food) {
        match self.ordered_food.iter().position(|x| x == &food) {
            Some(index) => {
                self.ordered_food.remove(index);
            }
            None => {
                println!("The food does not exist")
            }
        }
    }

    pub fn is_food_ordered(&self, food_name: &String) -> bool {
        for food in &self.ordered_food {
            if &food.name == food_name {
                return true;
            }
        }

        return false;
    }

    pub fn notify_customer(&self, cooked_food: &mut Food) {
        // TODO
    }

    pub fn calculate_price(&self) -> f32 {
        let mut price: f32 = 0.0;
        let mut ordered_food_iterator = self.ordered_food.iter().peekable();

        while ordered_food_iterator.peek().is_some() {
            price = price + ordered_food_iterator.next().unwrap().price;
        }

        return price;
    }

    pub fn is_all_food_eaten(&self) -> bool {
        for food in &self.ordered_food {
            if food.status != FoodStatus::EATEN {
                return false;
            }
        }
        return true;
    }
}
