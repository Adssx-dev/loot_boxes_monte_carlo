use crate::simulation_result;
use crate::simulator;
use crate::data_exporter;
use std::thread;


pub struct MonteCarloSimulator {
    number_of_simulations : u32,
    simulation_results : simulation_result::SimulationResult,
    simulator : simulator::Simulator,
    max_iterations_per_simulation : u32,
}

impl MonteCarloSimulator {
    pub fn init(simulations_number : u32, target_number : u32, max_iterations_per_simulation: u32) -> MonteCarloSimulator {
        MonteCarloSimulator {
            number_of_simulations : simulations_number,
            max_iterations_per_simulation : max_iterations_per_simulation,
            simulator : simulator::Simulator::init(target_number, max_iterations_per_simulation),
            simulation_results : simulation_result::SimulationResult::init(max_iterations_per_simulation as usize),
        }
    }

    pub fn simulate<T: data_exporter::DataExporter> (&mut self, number_of_steps : u32, thread_count : u32,  exporter : &mut T) -> &simulation_result::SimulationResult {
        let mut i : u32 = 0;

        let partial_simulations_count = self.number_of_simulations / number_of_steps;
        while i < number_of_steps{
            i = i + 1;
            let tmp_result = self.partial_simulate(partial_simulations_count, thread_count);
            self.simulation_results.add_all_results(&tmp_result);
            exporter.export(&(self.simulation_results));
        }
        return &self.simulation_results;
    }

    fn partial_simulate(&mut self, partial_simulation_size : u32, thread_count : u32) -> simulation_result::SimulationResult {
        let mut partial_result = simulation_result::SimulationResult::init(self.max_iterations_per_simulation as usize);
        let max_iterations_per_simulation = self.max_iterations_per_simulation;
        let mut handles = vec!();

        for i in 0..thread_count {
            let mut simulator = self.simulator.clone();
            handles.push(std::thread::spawn(move || parallel_simulate(&mut simulator, partial_simulation_size / thread_count, max_iterations_per_simulation))); 
        }

        for handle in handles {
            let tmp_result = handle.join().unwrap();
            partial_result.add_all_results(&tmp_result);
        }

        partial_result
    }

    
}

fn parallel_simulate(simulator : &mut simulator::Simulator ,number_of_simulations : u32, max_iterations_per_simulation : u32) -> simulation_result::SimulationResult {
    let mut my_simulator = simulator.clone();
    let mut i = 0;
    let mut partial_result = simulation_result::SimulationResult::init(max_iterations_per_simulation as usize);

    while i < number_of_simulations {
        i += 1;
        let result = my_simulator.simulate();
        partial_result.add_result(result as usize, 1);
    }

    return partial_result;
}