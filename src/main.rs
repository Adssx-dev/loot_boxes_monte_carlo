use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let max_iters = 100000;
    let mut i = 0;
    let mut all_sims : HashMap<u32, u32> = HashMap::new();
    
    while i < max_iters{
        i = i + 1;
        let last_sim = simulate();
        let score_of_sims = all_sims.get(&last_sim);
        let score = match score_of_sims{
            None => 1,
            Some(&sc) => sc + 1
        };
        all_sims.insert(last_sim, score);
    }

    let mut file = File::create("output.csv").unwrap();
    file.write_all(b"Number;Occurences");

    for elem in all_sims.iter() {
        let line = format!("\n{};{}", elem.0, elem.1);
        file.write(line.as_bytes());
    }

}

fn simulate() -> u32{

    let mut all_deps : Vec<i32>= Vec::new();
    let threshold = 1000;
    let mut it = 0;

    loop{
        if it > threshold{
            return it;
        }
        it = it + 1;
        
        let new_dep = rand::thread_rng().gen_range(1..97);
        if !all_deps.contains(&new_dep){
            all_deps.push(new_dep);
        }
        
        if all_deps.len() >= 96 {
            return it;
        }
    }

}
