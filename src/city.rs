// src/city.rs

#[derive(Debug, Clone)]


pub struct City {
    pub name: String,
    pub x: f64,
    pub y: f64,
}

pub struct Distance {
    pub city_1: String,
    pub city_2: String,
    pub dist: f64,
}

pub fn calculate_distance(c1: &City, c2: &City) -> f64 {
    ((c1.x - c2.x).powi(2) + (c1.y - c2.y).powi(2)).sqrt()
}

pub fn generate_permutations(mut cities: Vec<City>, number: usize, starting_city: &str) -> Vec<Distance> {
    
    
    let mut distances = Vec::new();




    for i in 0..cities.len(){   // Puts the chosen city as the first in the list

        if cities[i].name == starting_city{

            let temp: City = cities[0].clone();

            cities[0] = cities[i].clone();

            cities[i] = temp;

            break;

        }

    }



    println!("\nYour City Travel Order:\n");


    for i in 0..number-1 {


        println!("{}", &cities[i].name);

        let mut min_distance = f64::INFINITY;

        let mut k = 1;

        for j in (i + 1)..number {

            let distance = calculate_distance(&cities[i], &cities[j]);
            
            if distance < min_distance {

                min_distance = distance;

                k = j;

            }




        }


        distances.push(Distance {
            city_1: cities[i].name.clone(),
            city_2: cities[k].name.clone(),
            dist: min_distance,
        });


        let temp_city = cities[i+1].clone();

        cities[i+1] = cities[k].clone();

        cities[k] = temp_city;

    }

    distances.push(Distance {
        city_1: cities[number-1].name.clone(),
        city_2: cities[0].name.clone(),
        dist: calculate_distance(&cities[0], &cities[number-1]),
    });

    distances
}

/*pub fn minimum_distance(distances: Vec<Distance>) -> (Vec<Distance>, f64){


    for distance in distances {

        if distance.




    }



}*/




/*pub fn reorder_brute_force(distances: Vec<Distance>) -> (Vec<Distance>, f64) { // Outputs reordered list for shortest distance and total distance
    
    let mut temp_dist = Vec::new();

    for i in 0..distances.len() {

        for j in (i+1)..distances.len(){

            if distances[i].dist < distances[j].dist {


                temp_dist.push[i](Distance) {distances[i].clone()}

            }

            else {

                temp_dist[i] = distances[j].clone();

            }



        }

    }
    
    let total_dist = f64::default();

    for i in 0..temp_dist.len() {

        total_dist += temp_dist[i].dist;

    }
    

    
    
    (temp_dist, total_dist)


}*/
