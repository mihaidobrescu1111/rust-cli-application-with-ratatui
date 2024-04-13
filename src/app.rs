use std::error;

use crate::connection::{get_cities, get_temperature, get_forecast, CityInfo};

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub cities: Vec<String>,
    pub running: bool,
    pub index: i32,
    pub current_city: Option<CityInfo>,
    pub current_city_forecast: Option<Vec<CityInfo>>
}

impl App {
    /// Constructs a new instance of [`App`].
    pub async fn new() -> Self {
        
        let  mut app = Self {
            cities: [].to_vec(),
            running: true,
            index: 0,
            current_city: None,
            current_city_forecast: None,
        };

        app.cities = get_cities().await.unwrap();
        let current_city = get_temperature(app.cities[app.index as usize].clone()).await.unwrap();
        app.current_city = Some(current_city);
        let curren_city_forecast = get_forecast(app.cities[app.index as usize].clone()).await.unwrap();
        app.current_city_forecast = Some(curren_city_forecast);
        // Nu stiu ce are imi vine sa ma omor
        app
    }
    pub async fn get_temp(&mut self) {
        let current_city: String = self.cities.get(self.index as usize).unwrap().to_string();
        self.current_city = Some(get_temperature(current_city.clone()).await.unwrap());
        self.current_city_forecast = Some(get_forecast(current_city.clone()).await.unwrap());
    }
    pub async fn down(&mut self) {
        self.index = self.index + 1;
        if self.index == self.cities.len() as i32{
            self.index = 0;
        }
        self.get_temp().await;
    }
    pub async fn up(&mut self) {
        self.index = self.index -1;
        if self.index == -1 {
            self.index = self.cities.len() as i32 - 1;
        }
        self.get_temp().await;
    }
}
