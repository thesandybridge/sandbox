use plotters::prelude::*;
use anyhow::Result;

const OUT_FILE_NAME: &'static str = "images/histogram.png";
pub fn bar(data: &Vec<usize>) -> Result<()>{
    let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram Test", ("sans-serif", 50.0))
        .build_cartesian_2d((0..data.len()).into_segmented(), 0..10000)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Hours")
        .x_desc("Supply")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    println!("{:?}", data);

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(data.iter().map(|x| (*x, 1))),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'images' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    return Ok(());
}
