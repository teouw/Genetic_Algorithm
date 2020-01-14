extern crate matrice_lib;
extern crate city_position_lib;
extern crate rand;
use rand::Rng;
extern crate plotlib;
use plotlib::style::Line;

#[derive(Clone,Debug)]
pub struct Individual {
    pub path: Vec<usize>
}

#[derive(Clone,Debug)]
pub struct Population {
    pub ind: Vec<Individual>,
}

//imlp for Individuals
impl Individual {

    pub fn new(ids: Vec<usize>)-> Individual { //Create a new Individuals from a vec of usize (cities)
        Individual {
            path: ids,
        }
    }

    pub fn distance_between_cities(self, matrice: Vec<Vec<f32>>) -> f32{ //search the total distance of the cities in the Indi
        let mut dist = 0.0;
        for x in 0..(self.path.len() - 1){
            dist = dist + matrice[self.path[x]][self.path[x + 1]];
        }
        dist = dist + matrice[self.path[51]][self.path[0]];

        return dist;
    }

    fn crossover(self ,other :Individual) ->Individual{  //Mixing cities from a parent indi with a other to create a child
        let mut rng = rand::thread_rng();
        let ind_len = self.path.len();
        let city_blocked :usize = rng.gen_range(47, 52);
        let mut child :Individual =  Individual { path: vec![999; ind_len],};
        let mut cities_used :Vec<usize> = Vec::new();
        
        while cities_used.clone().len() != city_blocked{
            let index :usize = rng.gen_range(0, (ind_len as usize) - 1);
            if !cities_used.contains(&child.path[index as usize].clone()){
                child.path[index as usize] = self.path[index as usize].clone();
                cities_used.push(self.path[index as usize].clone());
            }
        }
        
        for i in other.path{
            if !cities_used.contains(&i) {
                let mut cmpt = 0;
                while child.path[cmpt] != 999{
                    cmpt = cmpt + 1;
                }
                child.path[cmpt] = i;
            }
        }
        return child;
    }

    fn mutation(&mut self,) -> Individual {  //Create a mutation by swaping 2 cities the indi given
        let mut rng = rand::thread_rng();
        for _i in 0..1{
            let mut rdn = rng.gen_range(0, 101);
            if rdn < 80{
                let mut x = rng.gen_range(0, self.path.len());
                let mut y = rng.gen_range(0, self.path.len());
                while x == y{
                    y = rng.gen_range(0, self.path.len());
                }
                let mut z = self.path[x];
                self.path[x] = self.path[y];
                self.path[y] = z;
            }
        }
        return self.clone();
    }

    fn adaptibilite(self, matrice: Vec<Vec<f32>>) -> f32{  //calculating the adaptability of the indi
        let ada = self.distance_between_cities(matrice)/100.0;
        return ada;
    }
}

//imlp for Population
impl Population{ 

    pub fn new(inds: Vec<Individual>) -> Population{  //Create a new Population from a vec of Inviduals
        Population{
            ind: inds,
        }
    }

    pub fn best_individuals(self, matrice: Vec<Vec<f32>>) -> Individual{  //searching for the best individuals in the population given
        let mut best_path = self.ind[0].clone().distance_between_cities(matrice.clone());
        let mut best_ind = self.ind[0].clone();
        
        for i in 1..self.ind.len(){
            let mut x = self.ind[i].clone().distance_between_cities(matrice.clone());
            if x < best_path{
                best_path = x;
                best_ind = self.ind[i].clone();
            }
        }
        return best_ind;
    }

    pub fn create_small_population(self) -> Vec<Individual>{ //Creating a random small pop from the Population given
        
        let mut small_pop = Vec::new();
        let mut rng = rand::thread_rng();
        let nbr_ind_start: usize;
        let nbr_ind_end: usize;

        nbr_ind_start = rng.gen_range(1, self.ind.len()/2);
        nbr_ind_end = rng.gen_range(nbr_ind_start + 1, self.ind.len());

        for i in nbr_ind_start..nbr_ind_end{
            small_pop.push(self.ind[i].clone());
        }

        return small_pop;
    }
}

//creating a population of 100 indi
pub fn create_population(first_pop: &mut bool, best_indiv: Population, matrice: Vec<Vec<f32>>) -> Vec<Individual>{
    let mut pop = Vec::new();

    if *first_pop == true{
        for _i in 0..100{
            let mut ind1 = randoms_individual();
            let mut p1 = Individual::new(ind1);
            pop.push(p1);
        }
    }
    else{
        pop.push(best_indiv.clone().best_individuals(matrice.clone()));
        for _i in 0..99{
            let mut ind2 = randoms_individual_from_old_best(best_indiv.clone().best_individuals(matrice.clone()));
            let mut p2 = Individual::new(ind2);
            pop.push(p2);
        }
    }
    return pop;
    
}

//creating a random indi
pub fn randoms_individual() -> Vec<usize>{
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    let mut indi = Vec::new();
    let mut x: usize;
    for i in 0..52{
        vec.push(i);
    }
    while vec.len() != 0{
        x = rng.gen_range(0, vec.len());
        indi.push(vec[x]);
        vec.remove(x);
    }
    return indi;
}

//function to start the algorithms
pub fn start(first_pop: &mut bool, last_best_path_vec: &mut Vec<Individual>, last_best_path: &mut f32, stop_cnt: &mut i32, last_best_indi: &mut Individual, matrice: Vec<Vec<f32>>, end: &mut bool){
     
    //the best adaptability calculated
    let mut _ecart_type_moy = 50.0;

    //bool to check if the best path was find and we can print the result only the last
    let mut show_last_only = false;

    //check if the best path is higher or greater than 10000
    let mut higher_than_10k: f32;
    
    //counter for the display
    let mut cnt_display = 0;

    //vec of the best individuals (the parents) to be crossover after when the vec == 2
    let mut vec_best_indi = Vec::new();

    //vec of the best adaptabilities (of the child) used to calculate the "ecart type / moy" when the vec == 20
    let mut vec_best_ada = Vec::new();;

    //new best pop will be the new vec of pop that contains all the best children of the parents
    let mut new_best_pop = Vec::new();

    //get the best indiv to give it to the next generation (population)
    let best_indi_for_next_generation = Population::new(last_best_path_vec.clone());

    //creating a population
    let pop = create_population(first_pop, best_indi_for_next_generation.clone(), matrice.clone());

    //clear the best path vec
    last_best_path_vec.clear();

    let mut population: Population;
    let mut new_population = Population::new(pop.clone());

    while *end == false{   //Continue to search the best path until the best individual can't be beaten
        //population will be generate at first than it will be the new_popilation of the children
        population = new_population.clone();

        //create a small population from the population
        let small_pop = Population::new(population.clone().create_small_population());

        //puttinf the best individuals of the small pop into a vec of best ind
        vec_best_indi.push(small_pop.best_individuals(matrice.clone()));

        if vec_best_indi.len() == 2{  //when we get 2 parents we call the crossover and the mutation function and we clear the vec
            let mut new_child = vec_best_indi[0].clone().crossover(vec_best_indi[1].clone());
            new_best_pop.push(new_child.mutation());

            let mut new_child2 = vec_best_indi[1].clone().crossover(vec_best_indi[0].clone());
            new_best_pop.push(new_child2.mutation());

            vec_best_indi.clear();
        }

        if new_best_pop.len() == 100{  //when we get 100 children we put the best adaptability of the best ind into a vec of best ada
            let pop_children = Population::new(new_best_pop.clone());
            let best_child = pop_children.clone().best_individuals(matrice.clone());
            let best_adaptability = best_child.clone().adaptibilite(matrice.clone());
            vec_best_ada.push(best_adaptability);

            new_population = Population::new(new_best_pop.clone()); //the new_population will become the new_best_pop of all the children
            new_best_pop.clear();
            last_best_path_vec.push(best_child);
        }

        if vec_best_ada.len() == 10{  //When we find 20 adaptabilities we find the "ecart type / moy" to check if the results are the same
            _ecart_type_moy = ecart_type_moy(vec_best_ada.clone());
            let x = Population::new(last_best_path_vec.clone());
            higher_than_10k = x.clone().best_individuals(matrice.clone()).distance_between_cities(matrice.clone());
            if higher_than_10k < *last_best_path{
                *last_best_indi = x.clone().best_individuals(matrice.clone());
                *last_best_path = higher_than_10k.clone();
                *stop_cnt = 0;
            }
            else{
                *stop_cnt += 1;
                if *stop_cnt == 50{
                    *end = true;
                    break;
                }
            }
            vec_best_ada.clear();

            //Display to update the best result
            if cnt_display == 0{
                println!("Best: {} Calculating best path.", last_best_path);
                cnt_display = cnt_display + 1;
            }else if cnt_display == 1{
                println!("Best: {} Calculating best path..", last_best_path);
                cnt_display = cnt_display + 1;
            }else{
                println!("Best: {} Calculating best path...", last_best_path);
                cnt_display = 0;
            }
            print!("{}[2J", 27 as char);
        }
  
        //if the ecart_type_moy is lower than 0.1 and the best individual can be beaten we call the start() function again
        if _ecart_type_moy < 0.1 && *end == false {
            *first_pop = false;
            show_last_only = false;
            start(first_pop, last_best_path_vec,last_best_path, stop_cnt, last_best_indi, matrice.clone(), end);
        }
        else{
            show_last_only = true;
        }

    }

    if show_last_only == true{
        let points = create_tuple(last_best_indi.clone(), matrice.clone());
        draw(points);
        println!("best individual: {:?}", last_best_indi);
        println!("best path: {:?}", last_best_path);
    }


}

//find the ecart_type_moy with the vec of best adaptabilites 
fn ecart_type_moy(adaptibilite: Vec<f32>) -> f32{
    let mut moy = 0.0;
    for i in 0..adaptibilite.len(){
        moy = moy + adaptibilite[i];
    }
    moy = moy/adaptibilite.len() as f32;

    let mut ecart_type = 0.0;
    for x in 0..adaptibilite.len(){
        ecart_type = ecart_type + city_position_lib::pow(adaptibilite[x] - moy, 2);
    }
    ecart_type = ecart_type.sqrt();

    return ecart_type;
}

//Create a new popilation with a part of the last best individual of the last population
pub fn randoms_individual_from_old_best(best_indi: Individual) -> Vec<usize>{
    let mut best_start_vec = Vec::new();

    let mut rng = rand::thread_rng();
    let mut x: usize;
    for i in 0..42{
        best_start_vec.push(best_indi.path[i]);
    }

    while best_start_vec.len() != 52{
        x = rng.gen_range(0, 52);
        if !best_start_vec.contains(&x){
            best_start_vec.push(x);
        }
    }
    //println!("bsv {:?}", best_start_vec);
    return best_start_vec;
}

pub fn create_tuple(ind: Individual, m: Vec<Vec<f32>>) -> Vec<(f64, f64)> {
     let v = ind.path.clone();
     let mut result: Vec<(f64, f64)> = vec!();
     for i in 0..v.len() {
          result.push(((m[i][0]) as f64, (m[i][1]) as f64));
     }
     return result;
}

pub fn draw(points: Vec<(f64, f64)>) {
    
    let l1 = plotlib::line::Line::new(&points)
          .style(plotlib::line::Style::new().colour("black"));
    let v = plotlib::view::ContinuousView::new().add(&l1);
    plotlib::page::Page::single(&v).save("tmp/line".to_owned()+&".svg".to_owned()).expect("saving svg");

}
