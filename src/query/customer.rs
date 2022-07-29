use std::{collections::HashMap, io};

use crate::{
    restaurant::{customer::Customer, restaurant::Restaurant},
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
    };

    customers.insert(username, new_customer);
    println!("Customer created!");
}

pub fn get_customer(
    customers: &mut HashMap<String, Customer>,
    restaurants: &mut HashMap<String, Restaurant>,
) {
    println!("Insert the customer name: ");

    let mut customer_username = String::new();
    input_str(&mut customer_username);

    let customer = customers.get_mut(&customer_username);

    match customer {
        Some(value) => {
            println!("{}", value.to_string());
            customer_detail_query(value, restaurants);
        }
        None => {
            println!("Cannot find the customer, please try again")
        }
    };
}

pub fn customer_detail_query(
    customer: &mut Customer,
    restaurants: &mut HashMap<String, Restaurant>,
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
            1 => order_food(customer, restaurants),
            2 => visit_restaurant(),
            3 => pay(),
            4 => leave_restaurant(),
            _ => break,
        }
    }
}

pub fn order_food(customer: &mut Customer, restaurants: &mut HashMap<String, Restaurant>) {
    if customer.is_visiting_restaurant() {
        let mut food_name = String::new();
        input_str(&mut food_name);
        let restaurant_name = customer.visiting_restaurant.as_ref().unwrap();
        let restaurant = restaurants.get(restaurant_name).unwrap();
        let menu = &restaurant.menu;
        customer.order_menu(&food_name, menu);
    } else {
        println!("You are currently not visiting any restaurant");
    }
}

pub fn visit_restaurant() {}

pub fn pay() {}

pub fn leave_restaurant() {}
