use std::path::Path;
use plotters::prelude::*;

use crate::data_exporter;
use crate::simulation_result;

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

impl data_exporter::DataExporter for ResultPlotter {
    fn export(&mut self, sim_result : &simulation_result::SimulationResult) -> Result<(), Box<dyn std::error::Error>> {
        let folder = Path::new(&(self.folder));
        let current_path = folder.join(Path::new(&(format!("{:04}", self.current_id) + ".png")));
        self.current_id += 1;
        
        let root = BitMapBackend::new(&current_path, (800, 600)).into_drawing_area();
        
        root.fill(&WHITE)?;

        let areas = root.split_by_breakpoints([944], [80]);

        let mut scatter_ctx = ChartBuilder::on(&areas[2])
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0f64..1020f64, 0f64..5000f64)?;
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
}
