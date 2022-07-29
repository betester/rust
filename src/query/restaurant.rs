use parse_duration::parse;
use std::{collections::HashMap, io};

use crate::restaurant::{
    chef::Chef,
    customer::Customer,
    food::Food,
    food_status::FoodStatus,
    food_type::{self, FoodType},
    menu::Menu,
    restaurant::Restaurant,
};

pub fn create_restaurant(restaurants: &mut HashMap<String, Restaurant>) {
    let mut name = String::new();
    let mut adress = String::new();
    let mut rating = String::new();
    let mut available_seats = String::new();
    let customers = Vec::<String>::new();
    let chefs = Vec::<u32>::new();
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

pub fn get_restaurant(
    restaurants: &mut HashMap<String, Restaurant>,
    chefs: &mut HashMap<u32, Chef>,
) {
    let mut restaurant_name = String::new();

    println!("Insert the restaurant name: ");

    io::stdin()
        .read_line(&mut restaurant_name)
        .expect("Failed to read input");

    let restaurant = restaurants.get_mut(&restaurant_name);

    match restaurant {
        Some(value) => {
            println!("Here are the restaurant detail :");
            println!("name: {}", value.name);
            println!("address: {}", value.adress);
            println!("rating: {}", value.rating);
            println!("available seats: {}", value.available_seats);
            println!("chefs: {:?}", value.chefs);
            println!("chefs: {:?}", value.menu);
            restaurant_detail_query(value, chefs);
        }
        None => {
            println!("Cannot find the restaurant, please try again")
        }
    };
}

pub fn restaurant_detail_query(restaurant: &mut Restaurant, chefs: &mut HashMap<u32, Chef>) {
    loop {
        println!("What do you want to do?: (insert number)");
        println!("1. Add food menu");
        println!("2. Add chef");
        println!("3. Remove chef");
        println!("4. Done");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");
        let user_input = user_input.trim().parse().expect("Please enter a number");

        match user_input {
            1 => add_food_menu(restaurant),
            _ => break,
        }
    }
}

pub fn add_food_menu(restaurant: &mut Restaurant) {
    let mut name = String::new();
    let mut price = String::new();
    let mut cooking_time_estimation = String::new();
    let mut eating_time_estimation = String::new();

    println!("Please enter the food name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    println!("Please enter the food price: (in number) ");
    io::stdin()
        .read_line(&mut price)
        .expect("Failed to read input");

    println!("Please enter the cooking time estimation: (in hour, minute, or second) ");
    io::stdin()
        .read_line(&mut cooking_time_estimation)
        .expect("Failed to read input");

    println!("Please enter the eating time estimation: (in hour, minute, or second) ");
    io::stdin()
        .read_line(&mut eating_time_estimation)
        .expect("Failed to read input");

    let price: f32 = price.trim().parse().expect("Please input a number");
    let cooking_time_estimation = parse(&cooking_time_estimation).unwrap().into();
    let eating_time_estimation = parse(&eating_time_estimation).unwrap().into();

    let mut food_type_obj: FoodType = FoodType::SeaFood;

    loop {
        let mut food_type = String::new();

        println!("Please enter the food type : ");
        println!("1. Seafood");
        println!("2. Airfood");
        println!("3. Landfood");

        io::stdin()
            .read_line(&mut food_type)
            .expect("Failed to read input");

        let food_type = food_type.trim().parse().expect("please input a number");

        match food_type {
            1 => {
                food_type_obj = FoodType::SeaFood;
                break;
            }
            2 => {
                food_type_obj = FoodType::AirFood;
                break;
            }
            3 => {
                food_type_obj = FoodType::LandFood;
                break;
            }
            _ => {
                println!("invalid food type");
            }
        }
    }

    let new_food = Food {
        name,
        price,
        cooking_time_estimation,
        eating_time_estimation,
        food_type: food_type_obj,
        status: FoodStatus::RAW,
    };

    restaurant.add_food_menu(new_food);
    println!("food has been added!");
}

pub fn add_chef(restaurant:  &mut Restaurant, chefs:  HashMap<u32, Chef>) {
    let mut chef_id = String::new();

    io::stdin().read_line(&mut chef_id).expect("Failed to read input");

    let chef_id = chef_id.trim().parse().expect("Please insert a number");
    let chef = chefs.get(&chef_id).unwrap();

    restaurant.add_chefs(chef_id);
}

pub fn remove_chef() {}
