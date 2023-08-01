use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct Weather {
    pub city: String,
    pub low_temperature: f32,
    pub high_temperature: f32,
    pub humidity: f32,
    pub condition: String,
}

#[derive(Debug, Deserialize)]
pub struct WeatherRequest {
    pub city: Option<String>,
}