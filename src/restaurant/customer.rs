use super::{restaurant::Restaurant, menu::Menu,order::Order};

pub struct Customer<'a> {
    money : f32,
    number_seat : u32,
    visiting_restaurant : Restaurant,

}

impl<'a> Customer<'a> {
    fn order_menu(&self,food_name : String,order : &'a Order,menu: &'a Menu) {
        match menu.get_food_by_name(food_name) {
            Some(i) => order.add_food(i),
            None => println!("There are no such food")
        }
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