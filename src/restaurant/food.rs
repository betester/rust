use super::food_type::FoodType;

pub struct Food {
    pub name : String,
    pub price : f32,
    pub cooking_time_estimation : f32,
    pub eating_time_estimation : f32,
    pub food_type : FoodType,
}


impl PartialEq for Food {
    fn eq(&self, other: &Self) -> bool {
        self.name != other.name
    }
}