use std::time::{ Duration};

use super::{food_type::FoodType, food_status::FoodStatus};
#[derive(Clone)]
#[derive(Debug)]
pub struct Food {
    pub name : String,
    pub price : f32,
    pub cooking_time_estimation : Duration,
    pub eating_time_estimation : Duration,
    pub food_type : FoodType,
    pub status : FoodStatus,
}

impl Food {
    pub fn change_food_status(&mut self, new_status : FoodStatus) {
        self.status = new_status;
    }
}

impl PartialEq for Food {
    fn eq(&self, other: &Self) -> bool {
        self.name != other.name
    }
}