use std::path::Path;
use std::error::Error;
use plotters::prelude::*;

use crate::data_exporter::DataExporter;
use crate::simulation_result::SimulationResult;

/// Plots the results as pictures than can be used with FFMPEG to generate a video
pub struct ResultPlotter {
    current_id : u32,
    folder: String,
}

impl ResultPlotter {
   pub fn init(path : String) -> ResultPlotter {
       ResultPlotter{
            folder : path,
            current_id : 0,
        }
    }
}

impl DataExporter for ResultPlotter {
    fn export(&mut self, sim_result : &SimulationResult) -> Result<(), Box<dyn Error>> {
        let mut title: String = "Possible outcomes = 95; Number of simulations=".to_owned();
        title.push_str(&format!("{:08}", sim_result.total_number_of_simulations()));
        
        let folder = Path::new(&(self.folder));
        let current_path = folder.join(Path::new(&(format!("{:04}", self.current_id) + ".png")));
        self.current_id += 1;
        
        let root = BitMapBackend::new(&current_path, (800, 600)).into_drawing_area();
        //root.titled(, )?;

        let number_of_results = sim_result.total_number_of_simulations() as f64;

        root.fill(&WHITE)?;

        let (upper, lower) = root.split_vertically(50);

        ChartBuilder::on(&upper)
            .caption("Monte Carlo simulation of opening loot boxes", ("arial", 48).into_font().color(&BLUE))
            .build_cartesian_2d(0..1, 0..1)?
            .configure_mesh()
            .draw()?;

        let mut scatter_ctx = ChartBuilder::on(&lower)
            .caption(title, ("arial", 25).into_font())
            .x_label_area_size(60)
            .y_label_area_size(80)
            .build_cartesian_2d(0f64..1020f64, 0f64..0.01f64)?;
            

        let gray = RGBColor(100, 100, 100);

        scatter_ctx
            .configure_mesh()
            .light_line_style(&WHITE)
            .x_label_style(("arial", 20).into_font().color(&gray))
            .y_label_style(("arial", 20).into_font().color(&gray))
            .x_label_formatter(&|v| format!("{:.0}", v))
            .y_label_formatter(&|v| format!("{:.3}", v))
            .x_desc("Number of lootboxes to get all possible outcomes")
            .y_desc("Probability")
            .axis_desc_style(("arial", 20))
            .draw()?;

        scatter_ctx.draw_series(
            sim_result.result_table.iter().enumerate()
                .map(|(x, y)| Circle::new((x as f64, (*y as f64)/number_of_results), 2.0, Into::<ShapeStyle>::into(&BLUE).filled())).collect::<Vec<_>>()
        )?;

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

        Ok(())
    }
}
