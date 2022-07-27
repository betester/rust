use super::food::Food;

pub struct Order {
    ordered_food: Vec<Food>,
}

impl Order {
    pub fn add_food(&mut self, food: Food) {
        if !self.ordered_food.contains(&food) {
            self.ordered_food.push(food);
        }
    }

    pub fn remove_food(&mut self, food: Food) {
        match self.ordered_food.iter().position(|x| x == &food) {
            Some(index) => {
                self.ordered_food.remove(index);
            }
            None => {
                println!("The food does not exist")
            }
        }
    }

    pub fn calculate_price(&self) -> f32 {
        let mut price: f32 = 0.0;
        let mut ordered_food_iterator = self.ordered_food.iter().peekable();

        while ordered_food_iterator.next().is_some() {
            price = price + ordered_food_iterator.next().unwrap().price;
        }

        return price;
    }
}
