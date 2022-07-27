use super::{restaurant::Restaurant, menu::Menu};

pub struct Customer {
    money : f32,
    number_seat : u32,
    visiting_restaurant : Restaurant,

}

impl Customer {
    fn order_menu(&self,food_name : String) {
        let menu = &self.visiting_restaurant.menu;
        
    }

    fn visit_restaurant(&self) {
        
    }

    fn leave_restaurant(&self) {

    }

    fn eat(&self) {
        
    }

    fn pay(&self) {

    }
}