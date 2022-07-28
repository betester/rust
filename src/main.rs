use std::collections::HashMap;
use std::io;

use restaurant::{food::Food, food_type::FoodType, menu::Menu, restaurant::Restaurant};

use crate::restaurant::{chef::Chef, customer::Customer};

pub mod restaurant;

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

    println!("Welcome to scuffed restaurant handler, here are the list of order that you can do: ( insert number )");
    println!("1. Create Restaurant");
    println!("2. Create Customer");
    println!("3. Create Chef");
    println!("4. Done");

    loop {
        let mut query = String::new();

        io::stdin()
            .read_line(&mut query)
            .expect("Failed to read input");

        let query: u32 = query.trim().parse().expect("Please input a number");

        match query {
            1 => create_restaurant(restaurants),
            2 => create_customer(customers),
            3 => create_chef(chefs),
            other => {break}
        }
    }
}

fn create_customer(customers: &mut HashMap<String, Customer>) {
    let mut username = String::new();
    let mut money = String::new();

    println!("please insert the username: ");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read input");

    println!("please insert the amount of money : ");
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
    println!("Customer created!");
}

fn create_chef(chefs: &mut HashMap<u32, Chef>) {
    const MAX_SPECIALTIES: u32 = 3;
    let mut chef_id = String::new();
    let mut food_specialties = Vec::<FoodType>::new();
    let mut max_order_taken = String::new();

    println!("please insert the chef id (it has to be unique");
    io::stdin()
        .read_line(&mut chef_id)
        .expect("Failed to read input");

    println!("please insert the amount of order that the chef can take");
    io::stdin()
        .read_line(&mut max_order_taken)
        .expect("Failed to read input");

    println!("Please select three of the specialty that the chef can have (insert number)");
    println!("1. Seafood");
    println!("2. Airfood");
    println!("3. Landfood");
    println!("4. Done");

    for i in 0..MAX_SPECIALTIES {
        let mut specialty_input = String::new();

        io::stdin()
            .read_line(&mut max_order_taken)
            .expect("Failed to read input");

        let specialty_input: u32 = specialty_input
            .trim()
            .parse()
            .expect("Please input a number");
        match specialty_input {
            1 => food_specialties.push(FoodType::SeaFood),
            2 => food_specialties.push(FoodType::AirFood),
            3 => food_specialties.push(FoodType::LandFood),
            4 => break,
            other => {
                println!("invalid input")
            }
        }
    }

    let chef_id: u32 = chef_id.trim().parse().expect("Please input a number");
    let max_order_taken: u32 = max_order_taken
        .trim()
        .parse()
        .expect("Please input a number");

    let new_chef = Chef {
        chef_id,
        food_specialties,
        max_order_taken,
        order_taken: 0,
    };

    chefs.insert(new_chef.chef_id, new_chef);
    println!("Chef created!");

}

fn create_restaurant(restaurants: &mut HashMap<String, Restaurant>) {
    let mut name = String::new();
    let mut adress = String::new();
    let mut rating = String::new();
    let mut available_seats = String::new();
    let customers = Vec::<&Customer>::new();
    let chefs = Vec::<&Chef>::new();
    let foods = Vec::<Food>::new();
    let menu = Menu { foods };

    println!("please insert the name of the restaurant");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    println!("please insert the adress of the restaurant");
    io::stdin()
        .read_line(&mut adress)
        .expect("Failed to read input");

    println!("please insert the rating of the restaurant");
    io::stdin()
        .read_line(&mut rating)
        .expect("Failed to read input");

    println!("please insert the available seats of the restaurant");
    io::stdin()
        .read_line(&mut available_seats)
        .expect("Failed to read input");

    let rating: u8 = rating.trim().parse().expect("Please input a number");
    let available_seats: usize = available_seats
        .trim()
        .parse()
        .expect("Please input a number");

    let new_restaurant = Restaurant {
        name,
        adress,
        rating,
        available_seats,
        customers,
        chefs,
        menu,
    };

    restaurants.insert(new_restaurant.name.clone(), new_restaurant);
    println!("Restaurant created!");
}
