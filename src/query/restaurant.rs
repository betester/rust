use std::{collections::HashMap, io};

use crate::restaurant::{restaurant::Restaurant, customer::Customer, chef::Chef, food::Food, menu::Menu};

pub fn create_restaurant(restaurants: &mut HashMap<String, Restaurant>) {
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

pub fn get_restaurant(restaurants: &mut HashMap<String, Restaurant>) {
    let mut restaurant_name = String::new();

    println!("Insert the restaurant name: ");

    io::stdin()
        .read_line(&mut restaurant_name)
        .expect("Failed to read input");

    let restaurant = restaurants.get(&restaurant_name);

    match restaurant {
        Some(value) => {
            println!("here are the restaurant json {:?}", restaurant)
        }
        None => {
            println!("Cannot find the restaurant, please try again")
        }
    };
}
