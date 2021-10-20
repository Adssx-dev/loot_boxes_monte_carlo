use rand::Rng;


struct Simulator {
    targetNumber : u32,
    results : Vec<bool>,
    
}

pub fn simulate() -> u32{

    let mut all_deps : Vec<i32>= Vec::new();
    let threshold = 1000;
    let mut it = 0;

    loop{
        if it > threshold{
            return it;
        }
        it = it + 1;
        
        let new_dep = rand::thread_rng().gen_range(1..97);
        if !all_deps.contains(&new_dep){
            all_deps.push(new_dep);
        }
        
        if all_deps.len() >= 96 {
            return it;
        }
    }

}

