use super::{food_type::FoodType, food::Food};

pub struct Chef {
    food_specialties : Vec<FoodType>,
    max_order_taken : u32,
}

impl Chef {
    fn cook_food(&self, food: Food) {
        
    }
}