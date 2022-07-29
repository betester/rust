use std::{io, str::FromStr};

pub fn input_str(input : &mut String) {
    io::stdin().read_line(input).expect("Failed to read input");
}


pub fn input_number<T : FromStr>(input : &mut String) -> T where T : FromStr {
    input_str(input);
    match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {panic!("Please input a number")}
    }
}

