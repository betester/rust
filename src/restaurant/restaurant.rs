use super::{customer::Customer, menu::Menu, chef::Chef};

pub struct Restaurant<'a> {
    pub name : String,
    pub adress : String,
    pub rating : String,
    pub available_seats : u32,
    pub customers : Vec<Customer<'a>>,
    pub chefs : Vec<Chef>,
    pub menu : Menu,
}

impl<'a> Restaurant<'a> {

    fn add_food_menu(&self) {

    }

    fn remove_food_menu(&self) {

    }

    fn add_customers(&self) {

    }

    fn remove_customers(&self) {

    }

    fn add_chefs(&self) {

    }

    fn remove_chefs(&self) {

    }

    fn add_waiters(&self) {

    }

    fn remove_waiters(&self) {

    }

    fn is_visitable(&self) -> bool {
    // Handle whether the available_seats is fulfilling
    // Handle if the waiters is fulfilling
    // Handle if the chefs is fulfilling
        false
    }


    

    pub fn set_rating(&mut self, rating: String) {
        self.rating = rating;
    }
}