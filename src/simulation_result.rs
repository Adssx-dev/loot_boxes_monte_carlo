
/// Struct used to store and manipulate the results of one or multiple simulations
#[derive(Clone)]
pub struct SimulationResult {
    pub result_table : Vec<u32>,
    simulation_count : u32,
    max_size : usize,
    errors : u32,
}

impl SimulationResult {
    pub fn init(max_simulation_size : usize) -> SimulationResult {
        SimulationResult {
            result_table : vec![0; max_simulation_size],
            simulation_count : 0,    
            max_size : max_simulation_size,
            errors : 0,
        }
    }

    /// Add a result to the total
    pub fn add_result(&mut self, id : usize, count:u32) {
        if id < self.max_size {
            self.result_table[id as usize] += count;
            self.simulation_count += count;
        }
        else {
            self.errors += count;
        }
    }

    /// Add a whole SimulationResult to this one
    pub fn add_all_results(&mut self, other : &SimulationResult) {
        for (key, value) in other.result_table.iter().enumerate() {
            self.add_result(key, *value);
        }
        self.errors += other.errors;
    }
    
    /// Calculates the total number of simulations, which is the number of simulations that worked + the number of ones that failed
    pub fn total_number_of_simulations(&self) -> u32 {
        self.errors + self.simulation_count
    }
}