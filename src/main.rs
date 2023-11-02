// Import necessary modules and libraries.
mod scraper;
mod scraper_utils;
mod writer;

use std::collections::HashMap;

// Import necessary structures and functions from the crate.
use crate::scraper::{parse_page, SurvivorsAtAgeTable};
use crate::writer::write_data;

// The main function that drives the program.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a HashMap to store the data for each year.
    // The key is the year, and the value is a SurvivorsAtAgeTable struct containing the data.
    let mut all_data: HashMap<u32, SurvivorsAtAgeTable> = HashMap::new();
    
    // Loop through the years from 1900 to 2100 in steps of 10.
    for year in (1900..=2100).step_by(10) {
        // Parse the page for the given year and store the data in the HashMap.
        all_data.insert(year, parse_page(year)?);
    }
    
    // Write the collected data to a file or other output.
    write_data(all_data)?;
    
    // Return Ok to indicate success.
    Ok(())
}
