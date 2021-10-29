use std::time::Instant;

mod monte_carlo_simulator;
mod simulation_result;
mod simulator;
mod result_plotter;
mod data_exporter;

fn main() {
    let max_iters = 10000000_f32;
    let num_of_targets = 101;
    let max_iterations_per_simulation = 1000;

    let timer = Instant::now();

    
    let mut plotter = result_plotter::ResultPlotter::init("data/images".to_string());

    let mut simulator = monte_carlo_simulator::MonteCarloSimulator::init(
        num_of_targets,
        max_iterations_per_simulation,
    );

    let vector = log_vector(500.0, max_iters, 10.0, 300);

    simulator.simulate(10, &mut plotter, &vector);


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
    for _i in 1..steps {
        current_log_value = current_log_value + delta;
        result.push(base.powf(current_log_value).ceil() as u32);
    }
    result.push(max.ceil() as u32);

    result
}

