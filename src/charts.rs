use std::collections::BTreeMap;

use plotters::{
    prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, IntoSegmentedCoord},
    series::{Histogram},
    style::{Color, RED, WHITE},
};

pub fn plot_histogram(histogram: &BTreeMap<i64, usize>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("histogram.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram Test", ("sans-serif", 50.0))
        .build_cartesian_2d((0u32..2000u32).into_segmented(), 0u32..20u32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    let data = [
        0u32, 1, 1, 1, 4, 2, 5, 7, 8, 6, 4, 2, 1, 8, 3, 3, 3, 4, 4, 3, 3, 3,
    ];

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(histogram.iter().map(|k| (*k.0 as u32, *k.1 as u32))),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    Ok(())
}
