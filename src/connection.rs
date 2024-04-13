use chrono::{DateTime, Local, Utc};
use crossterm::terminal::ClearType;
use reqwest::{Body, Client};
use serde::de::DeserializeOwned;
use serde_derive::{Serialize, Deserialize};
use chrono::serde::{ts_seconds_option, ts_seconds};

#[derive(Debug, Serialize, Deserialize)]
pub struct Conditions {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: f32,
    pub humidity: f32,
    pub clouds_pct: f32,
    pub wind: Wind,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wind {
    pub speed: f32,
    pub deg: f32,
    pub gust: Option<f32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    pub main_weather: String,
    pub description: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CityInfo {
    // TODO: define elements in the structure
    pub city: String,
    pub time: Option<String>,
    pub weather: Weather,
    pub conditions: Conditions,
    pub current_time: Option<String>,
    pub sunrise: Option<String>,
    pub sunset: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cities {
    pub cities: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Forecast {
    city: String,
    forecast: Vec<CityInfo>
}

/// Method that is handling the request to the openweather api,
/// parsing the response
///
/// IP: 34.116.205.113
/// Port: 3000
///
/// Returns weather details about a certain city
pub async fn get_temperature(city: String) -> Result<CityInfo, reqwest::Error>{
    let client = reqwest::Client::new();
    let response = client.post("http://34.116.205.113:3000/cities/current_weather")
    .header("Content-Type", "application/json")
    .body(std::format!("{{\"city\": \"{}\"}}", city))
    .send().await;

    match response {
        Ok(response) => {
            Ok(response.json::<CityInfo>().await.unwrap())
        },
        Err(error) => {
            Err(error)
        }
    }
}

pub async fn get_cities() -> Result<Vec<String>, reqwest::Error>{
    let client: Client = reqwest::Client::new();
    let response = client.get("http://34.116.205.113:3000/cities").send().await;
    match response {
        Ok(response) => {
            Ok(response.json::<Cities>().await.unwrap().cities)
        },
        Err(error) => {
            Err(error)
        }
    }
}

// pub async fn get_forecast(city: String) -> Result<Vec<CityInfo>, reqwest::Error>{
//     let client = reqwest::Client::new();
//     let response = client.post("http://34.116.205.113:3000/cities/forecast")
//     .header("Content-Type", "application/json")
//     .body(std::format!("{{\"city\": \"{}\"}}", city))
//     .send().await;

//     match response {
//         Ok(response) => {
//             Ok(response.json::<Forecast>().await.unwrap().forecast)
//         },
//         Err(error) => {
//             Err(error)
//         }
//     }
// }