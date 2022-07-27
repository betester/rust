use super::food::Food;

pub struct Menu {
    foods: Vec<Food>,
}

impl Menu {
    pub fn add_food(&mut self, food: Food) {
        if !self.foods.contains(&food) {
            self.foods.push(food);
        }
    }

    pub fn get_food_by_name(&self, food_name : String) -> Option<&Food> {
        for food in &self.foods {
            if food.name == food_name {
                return Some(&food);
            }
        }
        None
    }

    pub fn remove_food(&mut self, food: Food) {
        match self.foods.iter().position(|x| x == &food) {
            Some(index) => {
                self.foods.remove(index);
            }
            None => {
                println!("The food does not exist")
            }
        }
    }
}
