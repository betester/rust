use std::collections::HashMap;

use crate::{
    restaurant::{
        chef::Chef, customer::Customer, food::Food, order::Order, restaurant::Restaurant,
    },
    utils::input::{input_number, input_str},
};

pub fn create_customer(customers: &mut HashMap<String, Customer>) {
    let mut username = String::new();
    let mut money = String::new();

    println!("please insert the username: ");
    input_str(&mut username);

    println!("please insert the amount of money : ");
    let money: f32 = input_number(&mut money);

    let new_customer = Customer {
        username: username.clone(),
        money,
        visiting_restaurant: None,
        order: None,
        has_taken_order : false
    };

    customers.insert(username, new_customer);
    println!("Customer created!");
}

pub fn get_customer(
    customers: &mut HashMap<String, Customer>,
    restaurants: &mut HashMap<String, Restaurant>,
    chefs: &mut HashMap<u32, Chef>,
) {
    println!("Insert the customer name: ");

    let mut customer_username = String::new();
    input_str(&mut customer_username);

    let customer = customers.get_mut(&customer_username);

    match customer {
        Some(value) => {
            println!("{}", value.to_string());
            customer_detail_query(value, restaurants, chefs);
        }
        None => {
            println!("Cannot find the customer, please try again")
        }
    };
}

pub fn customer_detail_query(
    customer: &mut Customer,
    restaurants: &mut HashMap<String, Restaurant>,
    chefs: &mut HashMap<u32, Chef>,
) {
    loop {
        println!("What do you want to do?");
        println!("1. Order food");
        println!("2. Visit a restaurant");
        println!("3. Pay food");
        println!("4. Leave the restaurant");
        println!("5. Done");

        let mut user_input = String::new();
        let user_input: u8 = input_number(&mut user_input);

        match user_input {
            1 => order_food(customer, restaurants, chefs),
            2 => visit_restaurant(customer, restaurants),
            3 => pay(),
            4 => leave_restaurant(),
            _ => break,
        }
    }
}
// TODO: refactor to make it shorter
pub fn order_food(
    customer: &mut Customer,
    restaurants: &mut HashMap<String, Restaurant>,
    chefs: &mut HashMap<u32, Chef>,
) {
    if customer.is_visiting_restaurant() {
        let mut new_order = Order {
            ordered_food: Vec::<Food>::new(),
            customer: customer.username.clone(),
        };
        let restaurant_name = customer.visiting_restaurant.as_ref().unwrap();
        let restaurant = restaurants.get_mut(restaurant_name).unwrap();

        loop {
            let mut food_name = String::new();
            println!("Please enter the food name that you wanted to eat:");
            input_str(&mut food_name);
            let menu = &restaurant.menu;
            customer.order_food(&food_name, &menu, &mut new_order);

            println!("do you wish to add more order?");
            println!("1. Yes");
            println!("2. No");
            let mut add_order = String::new();
            let add_order: u8 = input_number(&mut add_order);

            match add_order {
                1 => continue,
                2 => break,
                _ => break,
            }
        }

        restaurant.handle_order(customer, &mut new_order, chefs);
        customer.set_order(new_order);
    } else {
        println!("You are currently not visiting any restaurant");
    }
}

pub fn visit_restaurant(customer: &mut Customer, restaurants: &mut HashMap<String, Restaurant>) {
    println!("Please enter the restaurant name");
    let mut restaurant_name = String::new();
    input_str(&mut restaurant_name);
    match restaurants.get_mut(&restaurant_name) {
        restaurant => {
            println!("You visited {}!", &restaurant_name);
            customer.visiting_restaurant = Some(restaurant_name);
            restaurant.unwrap().add_customers(customer.username.clone());
        }
        _ => (println!("The restaurant cannot be found, please try again.")),
    }
}

pub fn pay() {}

pub fn leave_restaurant() {}
