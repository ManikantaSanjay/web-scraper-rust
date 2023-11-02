// Import necessary modules and libraries.
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

// Import necessary structures from the crate.
use crate::SurvivorsAtAgeTable;
use serde_json;

// Function to write data to a file.
//
// This function takes a HashMap where the key is a u32 representing the year,
// and the value is a SurvivorsAtAgeTable struct containing the data for that year.
// The data is then serialized into JSON format and written to a file named "fileTables.json".
pub fn write_data(data: HashMap<u32, SurvivorsAtAgeTable>) -> std::io::Result<()> {
    // Create a new serde_json::Map to store the JSON data.
    let mut json_data = serde_json::Map::new();
    
    // Collect the keys from the HashMap and sort them.
    let mut keys = data.keys().collect::<Vec<_>>();
    keys.sort();
    
    // Loop through the sorted keys.
    for &key in keys {
        // Retrieve the value associated with the key.
        // Return an error if the key is not found.
        let value = data.get(&key).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Key not found"))?;
        
        // Convert the value to JSON format.
        // Return an error if the conversion fails.
        let json_value = serde_json::to_value(value).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        
        // Insert the key-value pair into the JSON map.
        json_data.insert(key.to_string(), json_value);
    }
    
    // Convert the JSON map to a pretty-printed JSON string.
    // Return an error if the conversion fails.
    let json_string = serde_json::to_string_pretty(&json_data).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    // Create a new file named "fileTables.json".
    // Return an error if the file creation fails.
    let mut file = File::create("fileTables.json")?;
    
    // Write the JSON string to the file.
    // Return an error if the write operation fails.
    write!(&mut file, "{}", json_string)?;
    
    // Return Ok to indicate success.
    Ok(())
}
