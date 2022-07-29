use std::collections::HashMap;
use std::io;

use restaurant::{ restaurant::Restaurant};

use crate::{restaurant::{chef::Chef, customer::Customer}, query::{restaurant::{create_restaurant, get_restaurant}, customer::{create_customer, get_customer}, chef::{create_chef, get_chef}}};

pub mod restaurant;
pub mod query;
pub mod utils;

fn main() {
    let mut customers = HashMap::<String, Customer>::new();
    let mut chefs = HashMap::<u32, Chef>::new();
    let mut restaurants = HashMap::<String, Restaurant>::new();

    handle_query(&mut chefs, &mut customers, &mut restaurants)
}

fn handle_query(
    chefs: &mut HashMap<u32, Chef>,
    customers: &mut HashMap<String, Customer>,
    restaurants: &mut HashMap<String, Restaurant>,
) {
    loop {
        println!(
            "Welcome to scuffed restaurant handler, 
        here are the list of order that you can do: ( insert number )"
        );
        println!("1. Create Restaurant");
        println!("2. Create Customer");
        println!("3. Create Chef");
        println!("4. Restaurant Detail");
        println!("5. Customer Detail");
        println!("6. Chef Detail");
        println!("7. Done");

        let mut query = String::new();

        io::stdin()
            .read_line(&mut query)
            .expect("Failed to read input");

        let query: u32 = query.trim().parse().expect("Please input a number");

        match query {
            1 => create_restaurant(restaurants),
            2 => create_customer(customers),
            3 => create_chef(chefs),
            4 => get_restaurant(restaurants,chefs),
            5 => get_customer(customers,restaurants),
            6 => get_chef(chefs),
            _ => break,
        }
    }
}