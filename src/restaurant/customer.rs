use super::{menu::Menu, order::Order, restaurant::Restaurant};

pub struct Customer {
    money: f32,
    number_seat: u32,
    visiting_restaurant: Option<Restaurant>,
}

impl Customer {
    fn order_menu(&self, food_name: String, order: Order, menu: Menu) {
        match menu.get_food_by_name(food_name) {
            Some(i) => order.add_food(i),
            None => println!("There are no such food"),
        }
    }

    fn visit_restaurant(&self, restaurant: &Restaurant) {}

    fn leave_restaurant(&self) {}

    fn eat(&self, order: &Order) {}

    fn pay(&self) {}
}
