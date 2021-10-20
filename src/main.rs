use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

mod simulator;

fn main() {
    let max_iters = 100000;
    let mut i = 0;
    let mut all_sims : HashMap<u32, u32> = HashMap::new();
    
    let now = Instant::now();

    while i < max_iters{
        i = i + 1;
        let last_sim = simulator::simulate();
        let score_of_sims = all_sims.get(&last_sim);
        let score = match score_of_sims{
            None => 1,
            Some(&sc) => sc + 1
        };
        all_sims.insert(last_sim, score);
    }

    println!("Calculated in {} seconds", now.elapsed().as_secs());

    //let mut file = File::create("output.csv").unwrap();
    //file.write_all(b"Number;Occurences");

    // for elem in all_sims.iter() {
    //     let line = format!("\n{};{}", elem.0, elem.1);
    //     file.write(line.as_bytes());
    // }

}


