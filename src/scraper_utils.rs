// Import necessary modules and libraries.
use std::sync::Mutex;
use std::time::Instant;

use lazy_static::lazy_static;
use reqwest::{blocking::Client, Error};

// Function to extract text content from a scraper::ElementRef.
//
// This function takes a reference to a scraper::ElementRef and returns a String
// containing the text content of the element. The text nodes of the element are
// collected into a Vec, joined into a single String, and then trimmed of any
// leading or trailing whitespace.
pub fn get_element_text(cell: &scraper::ElementRef) -> String {
    cell.text().collect::<Vec<_>>().join("").trim().to_string()
}

// Lazy static variables to hold shared resources and configuration.
lazy_static! {
    // Mutex to synchronize access to the last request time.
    static ref LAST_REQUEST_MUTEX: Mutex<Option<Instant>> = Mutex::new(None);
    
    // Duration to wait between requests to avoid overloading the server.
    static ref REQUEST_DELAY: std::time::Duration = std::time::Duration::from_millis(500);
    
    // reqwest::blocking::Client instance for making HTTP requests.
    static ref CLIENT: Client = Client::new();
}

// Function to make a throttled HTTP request.
//
// This function takes a URL as a String and returns a Result containing the
// response text or an Error. The function uses a Mutex to synchronize access
// to the last request time and ensure that there is a minimum delay between
// requests. If the time since the last request is less than the configured
// delay, the function sleeps for the remaining time before making the request.
pub fn do_throttled_request(url: &str) -> Result<String, Error> {
    let now = Instant::now();
    let mut last_request_mutex = LAST_REQUEST_MUTEX.lock().unwrap();
    
    if let Some(last_request) = *last_request_mutex {
        let duration = now.duration_since(last_request);
        if duration < *REQUEST_DELAY {
            drop(last_request_mutex); // Release the lock before sleeping
            std::thread::sleep(*REQUEST_DELAY - duration);
            last_request_mutex = LAST_REQUEST_MUTEX.lock().unwrap(); // Re-acquire the lock
        }
    }
    
    *last_request_mutex = Some(now);
    drop(last_request_mutex); // Release the lock before making the request
    
    let response = CLIENT.get(url).send()?;
    response.text()
}
