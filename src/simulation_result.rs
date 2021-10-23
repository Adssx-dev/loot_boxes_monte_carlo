use std::collections::HashMap;
use crate::simulation_result;

pub struct SimulationResult {
    pub result_table : HashMap<u32, u32>,
    simulation_count : u32,
}

impl SimulationResult {
    pub fn init() -> SimulationResult {
        simulation_result::SimulationResult {
            result_table : HashMap::new(),
            simulation_count : 0,
        }
    }

    pub fn add_result(&mut self, id : u32, count:u32) {
        let score_of_id = self.result_table.get(&id);
        let new_score = match score_of_id{
            None => 1,
            Some(sc) => sc + count
        };
        self.simulation_count += count;
        self.result_table.insert(id, new_score);
    }

    pub fn add_all_results(&mut self, other : &simulation_result::SimulationResult) {
        for (key, value) in &other.result_table {
            self.add_result(*key, *value);
        }
        self.simulation_count += other.simulation_count;
    }
}