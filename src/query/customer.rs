use std::{collections::HashMap, io};

use crate::{restaurant::customer::Customer, utils::input::{input_str, input_number}};

pub fn create_customer(customers: &mut HashMap<String, Customer>) {
    let mut username = String::new();
    let mut money = String::new();

    println!("please insert the username: ");
    input_str(&mut username);

    println!("please insert the amount of money : ");
    let money :f32 = input_number(&mut money);

    let new_customer = Customer {
        username: username.clone(),
        money,
        visiting_restaurant: None,
        order: None,
    };

    customers.insert(username, new_customer);
    println!("Customer created!");
}

pub fn get_customer(customers: &mut HashMap<String, Customer>) {
    println!("Insert the customer name: ");

    let mut customer_username = String::new();
    input_str(&mut customer_username);

    let customer = customers.get(&customer_username);

    match customer {
        Some(value) => {
            println!("{}", value.to_string());

        }
        None => {
            println!("Cannot find the customer, please try again")
        }
    };
}

pub fn customer_detail_query() {
    
}

pub fn order_food() {

}

pub fn visit_restaurant() {

}

pub fn pay() {

}

pub fn leave_restaurant() {

}