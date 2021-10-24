use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use rand::SeedableRng;

mod monte_carlo_simulator;
mod simulation_result;
mod simulator;
mod result_plotter;
mod data_exporter;

fn main() {
    let max_iters = 100000;
    let num_of_targets = 101;
    let max_iterations_per_simulation = 1000;

    let timer = Instant::now();

    let mut plotter = result_plotter::ResultPlotter::init("data/images".to_string());

    let mut simulator = monte_carlo_simulator::MonteCarloSimulator::init(
        max_iters,
        num_of_targets,
        max_iterations_per_simulation,
    );

    simulator.simulate(300, 10, &mut plotter);


    println!("Calculated in {} milliseconds", timer.elapsed().as_millis());



//     let mut file = File::create("output.csv").unwrap();
//     file.write_all(b"Number;Occurences");
        // for (pos, elem) in res.result_table.iter().enumerate() {
        //     let line = format!("\n{};{}", pos, elem);
        //     file.write(line.as_bytes());
//     }
}



