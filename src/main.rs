use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use plotters::prelude::*;
use rand::SeedableRng;

mod monte_carlo_simulator;
mod simulation_result;
mod simulator;

fn main() {
    let max_iters = 10000;
    let num_of_targets = 101;
    let max_iterations_per_simulation = 1000;

    let timer = Instant::now();

    let mut simulator = monte_carlo_simulator::MonteCarloSimulator::init(
        max_iters,
        num_of_targets,
        max_iterations_per_simulation,
    );

    let res = simulator.simulate();


    println!("Calculated in {} seconds", timer.elapsed().as_secs());

    plot_to_file(&res);

//     let mut file = File::create("output.csv").unwrap();
//     file.write_all(b"Number;Occurences");
        // for (pos, elem) in res.result_table.iter().enumerate() {
        //     let line = format!("\n{};{}", pos, elem);
        //     file.write(line.as_bytes());
//     }
}


pub fn plot_to_file(sim_result : &simulation_result::SimulationResult) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("data/1.png", (640, 480)).into_drawing_area();
    
    root.fill(&WHITE)?;

    let areas = root.split_by_breakpoints([944], [80]);

    let mut scatter_ctx = ChartBuilder::on(&areas[2])
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..1020f64, 0f64..500f64)?;
    scatter_ctx
        .configure_mesh()
        .draw()?;

    scatter_ctx.draw_series(
        sim_result.result_table.iter().enumerate()
            .map(|(x, y)| Circle::new((x as f64, *y as f64), 1.5, Into::<ShapeStyle>::into(&BLUE).filled())).collect::<Vec<_>>()
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    Ok(())
}
