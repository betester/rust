use super::{food::Food, food_status::FoodStatus, menu::Menu, order::Order};
use std::thread;
#[derive(Debug)]
pub struct Customer {
    pub username: String,
    pub money: f32,
    pub visiting_restaurant: Option<String>,
    pub order: Option<Order>,
}

impl Customer {

    pub fn to_string(&self) -> String {
        format!(
            "username : {}\n money : {} \n visiting_restaurant: {:?} \n order : {:?}",
            self.username, self.money, self.visiting_restaurant, self.order
        )
    }

    pub fn order_menu(&mut self, food_name: &String, menu: &Menu) {
        match &mut self.order {
            Some(value) => {
                value.add_food(food_name, menu);
                println!("{} has been ordered by {}", food_name, &self.username);
            }
            None => {
                let new_order = Order {
                    ordered_food: Vec::<Food>::new(),
                    customer: self.username.clone(),
                };
                self.order = Some(new_order);
                self.order_menu(food_name, menu);
            }
        }
    }

    pub fn is_visiting_restaurant(&self) -> bool {
        match &self.visiting_restaurant {
            Some(current_visiting) => true,
            None => false,
        }
    }

    pub fn visit_restaurant(&mut self, restaurant: String) {
        if !self.is_visiting_restaurant() {
            self.visiting_restaurant = Some(restaurant);
        }
    }

    pub fn leave_restaurant(&mut self) {
        if self.is_visiting_restaurant() {
            self.visiting_restaurant = None;
        }
    }

    pub fn eat(&self, food: &mut Food) {
        // need to use observer pattern to know when the food is cooked
        if food.status == FoodStatus::COOKED {
            thread::sleep(food.cooking_time_estimation);
            food.status = FoodStatus::EATEN;
        }
    }

    pub fn pay(&mut self) {
        match &mut self.order {
            Some(value) => {
                if value.is_all_food_eaten() {
                    let total_cost = value.calculate_price();
                    if self.money >= total_cost {
                        println!("Thank you for eating here, please come again")
                    } else {
                        println!("Get out!")
                    }
                }
            }
            None => println!("You have not ordered any food"),
        }
    }
}
