use std::{collections::HashMap, io};

use crate::restaurant::customer::Customer;

pub fn create_customer(customers: &mut HashMap<String, Customer>) {
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

pub fn get_customer(customers: &mut HashMap<String, Customer>) {
    println!("Insert the customer name: ");

    let mut customer_username = String::new();

    io::stdin()
        .read_line(&mut customer_username)
        .expect("Failed to read input");

    let customer = customers.get(&customer_username);

    match customer {
        Some(value) => {
            println!("here are the customer json {:?}", customer)
        }
        None => {
            println!("Cannot find the customer, please try again")
        }
    };
}