use super::{food::Food, food_status::FoodStatus, menu::Menu, order::Order};
use std::thread;
#[derive(Debug)]
pub struct Customer {
    pub username: String,
    pub money: f32,
    pub visiting_restaurant: Option<String>,
    pub order: Option<Order>,
    pub has_taken_order: bool,
    pub has_paid: bool,
}

impl Customer {
    pub fn to_string(&self) -> String {
        format!(
            "username : {}\n money : {} \n visiting_restaurant: {:?} \n order : {:?}",
            self.username, self.money, self.visiting_restaurant, self.order
        )
    }

    pub fn order_food(&mut self, food_name: &String, menu: &Menu, order: &mut Order) {
        // make sure that at least one order has been taken
        let result = menu.handle_order(food_name, order, &(&self.money - &order.calculate_price()));
        self.has_taken_order |= result;
        println!(
            "Your current money after buying : {}",
            &self.money - order.calculate_price()
        );
        order.print_ordered_food();
    }

    pub fn is_visiting_restaurant(&self) -> bool {
        match &self.visiting_restaurant {
            Some(_) => true,
            None => false,
        }
    }

    pub fn visit_restaurant(&mut self, restaurant: String) {
        if !self.is_visiting_restaurant() {
            self.visiting_restaurant = Some(restaurant);
        }
    }

    pub fn leave_restaurant(&mut self) -> bool {
        if self.is_visiting_restaurant() {
            if self.has_paid {
                self.visiting_restaurant = None;
                return true;
            } else {
                println!("You have not paid for the order yet");
                return false;
            }
        } else {
            println!("You have not visit any restaurant");
            return false;
        }
    }

    pub fn eat(&self, food: &mut Food) {
        // need to use observer pattern to know when the food is cooked
        if food.status == FoodStatus::COOKED {
            thread::sleep(food.cooking_time_estimation);
            println!("{} has finished eating the food", &self.username);
            food.status = FoodStatus::EATEN;
        }
    }

    pub fn set_order(&mut self, order: Order) {
        if order.ordered_food.len() == 0 {
            return;
        }
        self.order = Some(order);
    }

    pub fn pay(&mut self) {
        match &mut self.order {
            Some(value) => {
                if value.is_all_food_eaten() {
                    let total_cost = value.calculate_price();
                    if self.money >= total_cost {
                        println!("Thank you for paying");
                        self.has_paid = true;
                    } else {
                        println!("Get out!")
                    }
                }
            }
            None => {
                if self.has_taken_order {
                    println!("The food is still being processed, please wait");
                } else {
                    println!("You have not ordered any food");
                }
            }
        }
    }
}
