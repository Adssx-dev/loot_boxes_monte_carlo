use crate::simulation_result;
use crate::simulator;

pub struct MonteCarloSimulator {
    number_of_simulations : u32,
    simulation_results : simulation_result::SimulationResult,
    simulator : simulator::Simulator,
}

impl MonteCarloSimulator {
    pub fn init(simulations_number : u32, target_number : u32, max_iterations_per_simulation: u32) -> MonteCarloSimulator {
        MonteCarloSimulator {
            number_of_simulations : simulations_number,
            simulation_results : simulation_result::SimulationResult::init(),
            simulator : simulator::Simulator::init(target_number, max_iterations_per_simulation),
        }
    }

    pub fn simulate(&mut self) -> &simulation_result::SimulationResult {
        let mut i : u32 = 0;

        while i < self.number_of_simulations{
            i = i + 1;
            let result = self.simulator.simulate();
            self.simulation_results.add_result(result, 1);
        }
        return &self.simulation_results;
    }
}