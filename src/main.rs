use std::collections::HashMap;
use std::io;

use restaurant::food_type::FoodType;

use crate::restaurant::{chef::Chef, customer::Customer};

pub mod restaurant;

fn main() {
    let mut customers = HashMap::<String, Customer>::new();
    let mut chefs = HashMap::<String, Chef>::new();
    let mut restaurants = HashMap::<String, Chef>::new();
}

fn create_customer(customers: &mut HashMap<String, Customer>) {
    let mut username = String::new();
    let mut money = String::new();

    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read input");

    io::stdin()
        .read_line(&mut money)
        .expect("Failed to read input");

    let money: f32 = money.trim().parse().expect("Please input a number");

    let new_customer = Customer {
        username: username.clone(),
        money,
        visiting_restaurant: None,
        order: None,
    };

    customers.insert(username, new_customer);
}

fn create_chef(chefs: &mut HashMap<u32, Chef>) {
    const MAX_SPECIALTIES : u32 = 3;
    let mut chef_id = String::new();
    let mut food_specialties = Vec::<FoodType>::new();
    let mut max_order_taken = String::new();

    io::stdin()
    .read_line(&mut chef_id)
    .expect("Failed to read input");

    io::stdin()
    .read_line(&mut max_order_taken)
    .expect("Failed to read input");


    println!("Please select three of the specialty that the chef can have (insert number)");
    println!("1. Seafood");
    println!("2. Airfood");
    println!("3. Landfood");
    println!("4. Done");

    for i in 0..MAX_SPECIALTIES {
        let mut specialty_input  = String::new();

        io::stdin()
        .read_line(&mut max_order_taken)
        .expect("Failed to read input");

        let specialty_input: u32 = specialty_input
                                                                .trim()
                                                                .parse()
                                                                .expect("Please input a number");
       match specialty_input {
           1 => {food_specialties.push(FoodType::SeaFood)},
           2 => {food_specialties.push(FoodType::AirFood)},
           3 => {food_specialties.push(FoodType::LandFood)},
           4 => {break},
           other => {println!("invalid input")},
       }
    }

    let chef_id : u32 = chef_id.trim().parse().expect("Please input a number");
    let max_order_taken : u32 = max_order_taken.trim().parse().expect("Please input a number");

    let new_chef = Chef{
        chef_id,food_specialties,max_order_taken,order_taken : 0
    };

    chefs.insert(new_chef.chef_id, new_chef);


}

fn create_restaurant() {}
