use crate::simulation_result;

pub trait DataExporter {
    fn export(&mut self, sim_result : &simulation_result::SimulationResult) -> Result<(), Box<dyn std::error::Error>>;
}