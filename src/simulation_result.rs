use crate::simulation_result;
use std::cmp::Ordering;

pub struct SimulationResult {
    pub result_table : Vec<u32>,
    simulation_count : u32,
    max_size : usize,
    errors : u32,
}

impl SimulationResult {
    pub fn init(max_simulation_size : usize) -> SimulationResult {
        simulation_result::SimulationResult {
            result_table : vec![0; max_simulation_size],
            simulation_count : 0,    
            max_size : max_simulation_size,
            errors : 0,
        }
    }

    pub fn add_result(&mut self, id : usize, count:u32) {
        if id < self.max_size {
            self.result_table[id as usize] += count;
            self.simulation_count += count;
        }
        else {
            self.errors += count;
        }
    }

    pub fn add_all_results(&mut self, other : &simulation_result::SimulationResult) {
        for (key, value) in other.result_table.iter().enumerate() {
            self.add_result(key, *value);
        }
        self.errors += other.errors;
    }
    

    pub fn total_number_of_simulations(&self) -> u32 {
        self.errors + self.simulation_count
    }
}