use std::collections::BTreeMap;

use plotters::{
    prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, IntoSegmentedCoord},
    series::Histogram,
    style::{Color, RED, WHITE},
};

pub fn plot_histogram(histogram: &BTreeMap<i64, usize>) -> Result<(), Box<dyn std::error::Error>> {
    let x_min = match histogram.keys().min() {
        Some(min) => *min as u32,
        None => 0u32,
    };

    let x_max = match histogram.keys().max() {
        Some(min) => *min as u32,
        None => 0u32,
    };

    let y_min = match histogram.values().min() {
        Some(min) => *min as u32,
        None => 0u32,
    };

    let y_max = match histogram.values().max() {
        Some(min) => *min as u32,
        None => 0u32,
    };

    let root = BitMapBackend::new("histogram.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram", ("sans-serif", 50.0))
        .build_cartesian_2d((x_min..x_max).into_segmented(), y_min..y_max)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Amplitude")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(histogram.iter().map(|k| (*k.0 as u32, *k.1 as u32))),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    Ok(())
}
