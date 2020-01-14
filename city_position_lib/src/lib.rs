//lib to create the city and get positions from file
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//struct City with his x and y position
#[derive(Clone,Debug)]
pub struct City {
    pub x: f32,
    pub y: f32,
}

//impl for City
impl City {

    fn new(x_pos:f32, y_pos:f32,)-> City {  //Creating a new city with his x and y position and his id
        City {
            x: x_pos,
            y: y_pos,
        }
    }
}

// Allows equality checks 
impl PartialEq for City {
    fn eq(&self,other: &City) -> bool {
        
        if self.x == other.x && self.y == other.y{
            true
        }else{
            false
        }
    }
}

//Calculating the distance between 2 cities
pub fn distance_between_2_cities(city1:usize, city2: usize) -> f32{ 
    let _list_cities = get_positions_from_cities();
    let distance = (pow(_list_cities[city1].x - _list_cities[city2].x, 2) + pow(_list_cities[city1].y - _list_cities[city2].y, 2)).sqrt();
    return distance;
}

//Get positions from cities
pub fn get_positions_from_cities() -> Vec<City>{
    // creating a variable getting the contents of the file
    let path = Path::new("../../positions_city.txt");
    let mut _file_contents = File::open(&path).expect("error with the path");
    let mut _contents = String::new();
    _file_contents.read_to_string(&mut _contents).expect("error reading the file");
    let _line: Vec<String> = _contents.lines().map(|s| s.to_string()).collect();
    let mut _list_cities: Vec<City> = Vec::new();

    for x in 0.._line.len(){
        let _positions: Vec<String> = _line[x].split_whitespace().map(|s| s.to_string()).collect();  //splitting the line from the x pos and the y pos
        let _city_position = City::new(_positions[0].parse().unwrap(), _positions[1].parse().unwrap());  //creating a city from the x pos and y pos previously slipted
        _list_cities.push(_city_position.clone());  //adding the created city into a vec of all the cities
    }
    _list_cities

}

//Function to pow float
pub fn pow(x : f32, degree : i32) -> f32 {
    
    let mut x_calc : f32 = x;

    for _i in 1..degree.abs(){
        x_calc = x_calc * x
    }

    x_calc
}
