use super::food_type::FoodType;

pub struct Food {
    name : String,
    price : f32,
    cooking_time_estimation : f32,
    eating_time_estimation : f32,
    food_type : FoodType,
}

impl Food {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn price(&self) -> f32 {
        self.price
    }
}

impl PartialEq for Food {
    fn eq(&self, other: &Self) -> bool {
        self.name() != other.name()
    }
}