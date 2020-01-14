extern crate matrice_lib;
extern crate city_position_lib;
extern crate genetic_algorithms;


#[cfg(test)]
mod tests {

    //check matrice
    #[test]
    fn test_generate_matrice1() {
        let matrice = matrice_lib::generate_matrice();
        assert_eq!(matrice[10][20], 1314.1727);
    }

    //check matrice
    #[test]
    fn test_generate_matrice2() {
        let matrice = matrice_lib::generate_matrice();
        assert_eq!(matrice[35][51], 1116.3557);
    }

    //check matrice
    #[test]
    fn test_distance_between_2_cities1() {
        let city1 = 44;
        let city2 = 16;
        assert_eq!(city_position_lib::distance_between_2_cities(city1, city2), 436.5776);
    }

    //check matrice
    #[test]
    fn test_get_positions_from_cities1() {
        let _list_cities = city_position_lib::get_positions_from_cities();
        assert_eq!(_list_cities[2], city_position_lib::City { x: 345.0, y: 750.0 });
    }

    //check matrice
    #[test]
    fn test_distance_between_cities1() {
        let matrice = matrice_lib::generate_matrice();
        let ind :genetic_algorithms::Individual =  genetic_algorithms::Individual { path: vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51]};
        assert_eq!(ind.distance_between_cities(matrice.clone()), 22205.621);
    }
}
