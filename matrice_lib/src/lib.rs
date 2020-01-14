//lib to create the matrice for all the distances between cities
extern crate city_position_lib;

//Generation of the matrice from the positions of the cities
pub fn generate_matrice() -> Vec<Vec<f32>>{
    let mut matrice: Vec<Vec<f32>> = Vec::new();
    let mut distance: Vec<f32> = Vec::new();
    let _list_cities = city_position_lib::get_positions_from_cities();

    for i in 0..52{
        for j in 0..52{
            distance.push(city_position_lib::distance_between_2_cities(i, j)); //for every 2 cities well call the function to calculate their distnace
        }
        matrice.push(distance.clone());
        distance.clear();
    }

    //display_matrice(matrice.clone());
    return matrice;
}

//Functoin to display the matrice if needed
pub fn display_matrice(matrice: Vec<Vec<f32>>){
    for x in 0..52{
        for z in 0..52{
            if z == 52{
                println!("{}", matrice[x][z]);
            }
            else{
                print!("{} ", matrice[x][z]);
            }
        }
    }
}



