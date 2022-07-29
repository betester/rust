use super::{food::Food, order::Order};
#[derive(Debug)]
pub struct Menu {
    pub foods: Vec<Food>,
}

impl Menu {
    pub fn add_food(&mut self, food: Food) {
        if !self.foods.contains(&food) {
            self.foods.push(food);
        }
    }

    pub fn handle_order(&self, food_name: &String, order: &mut Order, customer_money: &f32) {
        match self.get_food_by_name(food_name) {
            Some(food) => {
                if customer_money - &food.price >= 0. {
                    order.add_food(food.clone())
                } else {
                    println!("Sorry your money is not enough");
                }
            }
            None => println!("The food not on the menu"),
        }
    }

    pub fn get_food_by_name(&self, food_name: &String) -> Option<&Food> {
        for food in &self.foods {
            if &food.name == food_name {
                return Some(&food);
            }
        }
        None
    }

    pub fn remove_food(&mut self, food: Food) {
        match self.foods.iter().position(|x| x == &food) {
            Some(index) => {
                self.foods.remove(index);
            }
            None => {
                println!("The food does not exist")
            }
        }
    }
}
