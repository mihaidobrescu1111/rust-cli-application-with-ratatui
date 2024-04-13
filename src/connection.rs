use chrono::{DateTime, Local};

struct CityInfo {
    // TODO: define elements in the structure
}

/// Method that is handling the request to the openweather api,
/// parsing the response
///
/// IP: 34.116.205.113
/// Port: 3000
///
/// Returns weather details about a certain city
pub fn get_temperature(city: String) {
    match reqwest::blocking::get("") {
        Ok(response) => {
            // Check status code
            // Parse response
        },
        Err(error) => {
            // Handle error
        }
    }
}