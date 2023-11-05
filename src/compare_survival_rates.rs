use charts::{Chart, VerticalBarView, AxisPosition, ScaleBand, ScaleLinear, LineSeriesView, MarkerType, PointLabelPosition, Color};
use serde::Deserialize;
use serde_json::from_str;
use std::fs::read_to_string;
use ndarray::Array1;
use ndarray_stats::SummaryStatisticsExt;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Data {
    #[serde(flatten)]
    years: HashMap<String, YearData>,
}

#[derive(Debug, Deserialize)]
struct YearData {
    female: Vec<f64>,
    male: Vec<f64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_content = read_to_string("fileTables.json")?;
    let data: Data = from_str(&file_content)?;

    generate_first_graph(&data)?;
    generate_second_graph(&data)?;

    Ok(())
}

fn generate_first_graph(data: &Data) -> Result<(), Box<dyn std::error::Error>> {
    let year_data = data.years.get("2100").unwrap();
    let female_rates = Array1::from(year_data.female.clone());
    let male_rates = Array1::from(year_data.male.clone());
    let differences = &female_rates - &male_rates;

    let mean_diff = differences.mean().unwrap();
    let var_diff = differences.var(0.);
    let std_diff = var_diff.sqrt();

    println!("Mean difference: {}", mean_diff);
    println!("Variance of differences: {}", var_diff);
    println!("Standard deviation of differences: {}", std_diff);

    // Define chart related sizes.
    let width = 800;
    let height = 600;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    // Create a band scale that maps categories to values in the [0, availableWidth] range.
    let x = ScaleBand::new()
        .set_domain(vec![String::from("Female"), String::from("Male")])
        .set_range(vec![0, width - left - right])
        .set_inner_padding(0.1)
        .set_outer_padding(0.1);

    // Create a linear scale that will interpolate values in [0, 1] range to corresponding
    // values in [availableHeight, 0] range.
    let y = ScaleLinear::new()
        .set_domain(vec![0.0, 1.0])
        .set_range(vec![height - top - bottom, 0]);

    // Create VerticalBar view that is going to represent the data as vertical bars.
    let view = VerticalBarView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .load_data(&vec![
            ("Female", female_rates.mean().unwrap() as f32),
            ("Male", male_rates.mean().unwrap() as f32),
        ])
        .unwrap();

    // Generate and save the chart.
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Survival Rates for Year 2100"))
        .add_view(&view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Survival Rate")
        .add_bottom_axis_label("Gender")
        .save("survival-rates-chart.svg")
        .unwrap();

    Ok(())
}

fn generate_second_graph(data: &Data) -> Result<(), Box<dyn std::error::Error>> {
    // Define chart related sizes.
    let width = 1800;
    let height = 600;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    // Create a linear scale for the x-axis
    let x = ScaleLinear::new()
        .set_domain(vec![1900_f32, 2100_f32])
        .set_range(vec![0, width - left - right]);

    // Create a linear scale for the y-axis
    let y = ScaleLinear::new()
        .set_domain(vec![0_f32, 100_f32]) // Assuming survival rates are between 0 and 100
        .set_range(vec![height - top - bottom, 0]);

    let mut female_data = Vec::new();
    let mut male_data = Vec::new();

    for year in (1900..=2100).step_by(10) {
        let year_str = year.to_string();
        if let Some(year_data) = data.years.get(&year_str) {
            let female_rates = Array1::from(year_data.female.clone());
            let male_rates = Array1::from(year_data.male.clone());

            let female_mean = female_rates.mean().unwrap_or(0.0) * 100.0;
            let male_mean = male_rates.mean().unwrap_or(0.0) * 100.0;
            
            // Round the mean values to two decimal places
            let female_mean_rounded = format!("{:.2}", female_mean).parse::<f32>().unwrap();
            let male_mean_rounded = format!("{:.2}", male_mean).parse::<f32>().unwrap();

            println!("Year: {}, Female mean: {:.2}%, Male mean: {:.2}%", year, female_mean_rounded, male_mean_rounded);

            female_data.push((year as f32, female_mean_rounded));
            male_data.push((year as f32, male_mean_rounded));
        }
    }

    // Create Line series view for female data
    let female_line_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_marker_type(MarkerType::Square)
        .set_colors(Color::color_scheme_dark())
        .set_label_position(PointLabelPosition::N)
        .set_custom_data_label("Female".to_owned())
        .load_data(&female_data).unwrap();

    // Create Line series view for male data
    let male_line_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_marker_type(MarkerType::Circle)
        .set_colors(Color::color_scheme_10())
        .set_label_position(PointLabelPosition::S)
        .set_custom_data_label("Male".to_owned())
        .load_data(&male_data).unwrap();

    // Generate and save the chart
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Mean Survival Rate Comparison"))
        .add_view(&female_line_view)
        .add_view(&male_line_view)
        .add_legend_at(AxisPosition::Right)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Survival Rate (%)")
        .add_bottom_axis_label("Year")
        .save("survival-rate-comparison-male-female-percentage.svg").unwrap();

    Ok(())
}



