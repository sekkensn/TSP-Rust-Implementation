// src/main.rs

mod utils;
mod city;



fn main() {


    let filename = "src/default_cities.txt";
    let cities = utils::read_cities_from_file(filename);
    println!("Cities loaded:");

    for city in &cities {
        println!("{}, ({}, {})", city.name, city.x, city.y);
    }

    let starting_city = utils::prompt_user_for_starting_city("\nEnter the starting city:", &cities);
    println!("Starting city: {}", starting_city);

    let num_cities = utils::prompt_user_for_number("\nEnter the number of cities:");
    println!("Number of cities: {}", num_cities);

    let distances = city::generate_permutations(cities.clone(),num_cities.try_into().unwrap(), &starting_city);

    let mut total_dist = f64::default();

    println!("\nShortest path from starting city:\n");


    for distance in &distances {
        println!("From City {} to City {} - Distance {:.0} km", distance.city_1, distance.city_2, distance.dist*100.0);
        total_dist += distance.dist;
    }

    println!("\nTotal Distance: {:.0} km", total_dist*100.0);
    

}
