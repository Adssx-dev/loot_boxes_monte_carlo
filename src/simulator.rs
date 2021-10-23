use rand::Rng;

pub struct Simulator {
    target_number: u32,
    max_iterations : u32,
    results: Vec<bool>,
    number_of_valid_elements: u32,
}

impl Simulator {
    pub fn init(number: u32, iterations: u32) -> Simulator {
        Simulator {
            target_number: number,
            max_iterations: iterations,
            number_of_valid_elements:0,
            results: vec![false; number as usize],
        }
    }

    pub fn simulate(&mut self) -> u32{
        self.reset();
        let mut it = 0;
        loop {
            it = it + 1;

            if it > self.max_iterations {
                return it;
            }

            let new_element = rand::thread_rng().gen_range(0..(self.target_number));

            if ! self.results[new_element as usize]  {
                self.results[new_element as usize] = true;
                self.number_of_valid_elements += 1;
                if self.number_of_valid_elements == self.target_number {
                    return it;
                }
            }

        }
    }

    fn reset(&mut self) {
        self.number_of_valid_elements = 0;
        self.results = vec![false; self.target_number as usize];
    }
}

