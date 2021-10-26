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

    let vector = log_vector(100.0, 10000000.0, 10.0, 300);

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



fn log_vector(min : f32, max : f32, base : f32, steps : u32) -> Vec<u32> {
    let log_min = min.log(base);
    let log_max = max.log(base);
    let delta = (log_max - log_min) / (steps as f32);

    let mut result = vec![];
    let mut current_log_value = log_min;

    result.push(min.ceil() as u32);
    for i in 1..steps {
        current_log_value = current_log_value + delta;
        result.push(base.powf(current_log_value).ceil() as u32);
    }
    result.push(max.ceil() as u32);

    result
}

