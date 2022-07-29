use super::{food_status::FoodStatus, menu::Menu, order::Order, food::Food};
use std::thread;
#[derive(Debug)]
pub struct Customer {
    pub username : String,
    pub money: f32,
    pub visiting_restaurant: Option<String>,
    pub order: Option<Order>,
}

impl Customer {
    pub fn order_menu(&mut self, food_name: &String, menu: &Menu) {
        
        match &mut self.order {
            Some(value) => {
                if value.is_food_ordered(&food_name) {
                    match menu.get_food_by_name(food_name) {
                        Some(i) => {
                            value.add_food(i.clone());
                            println!("{} has been ordered by {}",food_name,&self.username);
                        },
                        None => println!("There are no such food"),
                    }
                }
            },
            None => println!("No order has been created")
        }

    
    }

    pub fn visit_restaurant(&mut self, restaurant: String) {
        match &self.visiting_restaurant {
            Some(visited_restaurant) => {
                println!("You are currently visiting {}", visited_restaurant)
            }
            None => self.visiting_restaurant = Some(restaurant),
        }
    }

    pub fn leave_restaurant(&mut self) {
        match &self.visiting_restaurant {
            Some(visited_restaurant) => self.visiting_restaurant = None,
            None => println!("You are not visiting any restaurant"),
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
            },
            None => println!("You have not ordered any food")
            
        }
    }
}
