use crate::data_exporter::DataExporter;
use crate::simulation_result::SimulationResult;
use crate::simulator::Simulator;
use std::cmp;
use std::thread;

/// Struct to loop on simulation, aggregate results and output them
pub struct MonteCarloSimulator {
    simulation_results: SimulationResult,
    simulator: Simulator,
    max_iterations_per_simulation: u32,
}

impl MonteCarloSimulator {
    pub fn init(
        target_number: u32,
        max_iterations_per_simulation: u32,
    ) -> MonteCarloSimulator {
        MonteCarloSimulator {
            max_iterations_per_simulation: max_iterations_per_simulation,
            simulator: Simulator::init(target_number, max_iterations_per_simulation),
            simulation_results: SimulationResult::init(
                max_iterations_per_simulation as usize,
            ),
        }
    }

    /// Starts the simulation and outputs results with the DataExporter according to steps specified via update_vector
    pub fn simulate<T: DataExporter>(
        &mut self,
        thread_count: u32,
        exporter: &mut T,
        update_vector : &Vec<u32>
    ) -> &SimulationResult {

        let mut previous = 0;

        for current in update_vector.iter() {
            let delta = current - previous;

            let tmp_result = self.partial_simulate(delta, thread_count);
            self.simulation_results.add_all_results(&tmp_result);
            exporter.export(&(self.simulation_results)).expect("Could not plot result");

            previous = *current;
        }

        return &self.simulation_results;
    }

    /// Starts the simulation for the given step.
    /// Spawns multiple threads to improve speed
    fn partial_simulate(
        &mut self,
        partial_simulation_size: u32,
        thread_count: u32,
    ) -> SimulationResult {
        let mut partial_result =
            SimulationResult::init(self.max_iterations_per_simulation as usize);
        let max_iterations_per_simulation = self.max_iterations_per_simulation;
        let mut handles = vec![];

        let mut remaining = partial_simulation_size;
        let mut iters_per_thread = partial_simulation_size / thread_count;

        if partial_simulation_size % thread_count > 0 {
            iters_per_thread += 1;
        }

        for _i in 0..thread_count {
            let mut simulator = self.simulator.clone();
            handles.push(thread::spawn(move || {
                parallel_simulate(
                    &mut simulator,
                    cmp::min(iters_per_thread, remaining),
                    max_iterations_per_simulation,
                )
            }));
            if remaining > iters_per_thread{
                remaining -= iters_per_thread;
            }
            else {
                remaining=0;
            }
        }

        for handle in handles {
            let tmp_result = handle.join().unwrap();
            partial_result.add_all_results(&tmp_result);
        }

        partial_result
    }
}

/// Actual simulation function that can be started in a thread
fn parallel_simulate(
    simulator: &mut Simulator,
    number_of_simulations: u32,
    max_iterations_per_simulation: u32,
) -> SimulationResult {
    let mut my_simulator = simulator.clone();
    let mut i = 0;
    let mut partial_result =
        SimulationResult::init(max_iterations_per_simulation as usize);

    while i < number_of_simulations {
        i += 1;
        let result = my_simulator.simulate();
        partial_result.add_result(result as usize, 1);
    }

    return partial_result;
}
