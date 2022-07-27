use super::{food_status::FoodStatus, menu::Menu, order::Order, restaurant::Restaurant, food::Food};
use std::thread;

pub struct Customer<'a> {
    pub money: f32,
    pub number_seat: u32,
    pub visiting_restaurant: Option<&'a Restaurant<'a>>,
    pub order: Order<'a>,
}

impl<'a> Customer<'a> {
    fn order_menu(&mut self, food_name: String, menu: Menu) {
        if self.order.is_food_ordered(&food_name) {
            match menu.get_food_by_name(food_name) {
                Some(i) => self.order.add_food(i.clone()),
                None => println!("There are no such food"),
            }
        }
    }

    fn visit_restaurant(&mut self, restaurant: &'a Restaurant) {
        match self.visiting_restaurant {
            Some(visited_restaurant) => {
                println!("You are currently visiting {}", visited_restaurant.name)
            }
            None => self.visiting_restaurant = Some(restaurant),
        }
    }

    fn leave_restaurant(&mut self) {
        match self.visiting_restaurant {
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

    fn pay(&self) {
        if self.order.is_all_food_eaten() {
            let total_cost = self.order.calculate_price();
            if self.money >= total_cost {
                println!("Thank you for eating here, please come again")
            } else {
                println!("Get out!")
            }
        }
    }
}
