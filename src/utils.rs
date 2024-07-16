// src/utils.rs

use std::fs::File;
use std::io::{self, Write, BufRead, BufReader};

use crate::city::City;

pub fn read_cities_from_file(filename: &str) -> Vec<City> {
    let filename = if filename.is_empty() {
        "src/default_cities.txt"
    } else {
        filename
    };

    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            panic!("Failed to open file {}: {}", filename, err);
        }
    };

    let reader = BufReader::new(file);
    let mut cities = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

            if parts.len() >= 3 {
                let name = parts[0].to_string();
                let x = parts[1].parse().expect("Invalid latitude");
                let y = parts[2].parse().expect("Invalid longitude");

                let city = City { name, x, y };
                cities.push(city);
            }
        }
    }

    cities
}

pub fn prompt_user_for_number(prompt: &str) -> usize {
    use std::io::{self, Write};

    loop {
        print!("{} ", prompt);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) if num > 0 => return num,
            Ok(_) => println!("Please enter a positive number."),
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}


pub fn prompt_user_for_starting_city(prompt: &str, cities: &[City]) -> String {

    loop {
        print!("{} ", prompt);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        if cities.iter().any(|city| city.name == input) {
            return input.to_string();
        } else {
            let city_names: Vec<&str> = cities.iter().map(|city| city.name.as_str()).collect();
            println!("Invalid city. Please enter a city from the list: {:?}", city_names);
        }
    }
}