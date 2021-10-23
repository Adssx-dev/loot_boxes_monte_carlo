use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

mod monte_carlo_simulator;
mod simulation_result;
mod simulator;

fn main() {
    let max_iters = 100000;
    let num_of_targets = 101;
    let max_iterations_per_simulation = 1000;

    let timer = Instant::now();

    let mut simulator = monte_carlo_simulator::MonteCarloSimulator::init(
        max_iters,
        num_of_targets,
        max_iterations_per_simulation,
    );

    let res = simulator.simulate();


    println!("Calculated in {} seconds", timer.elapsed().as_secs());

    let mut file = File::create("output.csv").unwrap();
    file.write_all(b"Number;Occurences");

    for elem in res.result_table.iter() {
        let line = format!("\n{};{}", elem.0, elem.1);
        file.write(line.as_bytes());
    }
}
