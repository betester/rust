use super::{customer::Customer, menu::Menu, chef::Chef, waiter::Waiter};

pub struct Restaurant {
    name : String,
    adress : String,
    rating : String,
    available_seats : u32,
    customers : Vec<Customer>,
    chefs : Vec<Chef>,
    waiters : Vec<Waiter>,
    menu : Menu,
}

impl Restaurant {
    fn is_visitable(&self) -> bool {
    // Handle whether the available_seats is fulfilling
    // Handle if the waiters is fulfilling
    // Handle if the chefs is fulfilling
        false
    }

    
    
}