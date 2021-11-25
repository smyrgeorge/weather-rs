use serde_derive::Deserialize;

// Definitions for
// https://api.openweathermap.org/

#[derive(Default, Debug, Deserialize)]
pub struct WeatherResult {
    pub id: i32,
    pub dt: i32,
    pub base: String,
    pub timezone: i16,
    pub name: String,
    pub cod: i16,
    pub visibility: i16,

    pub weather: Vec<Weather>,
    pub coord: Coordinates,
    pub wind: Wind,
    pub clouds: Clouds,
    pub sys: Sys,
}

#[derive(Default, Debug, Deserialize)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Coordinates {
    lon: f32,
    lat: f32,
}

#[derive(Default, Debug, Deserialize)]
pub struct Main {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: i16,
    humidity: i16,
}

#[derive(Default, Debug, Deserialize)]
pub struct Wind {
    speed: f32,
    deg: i16,
}

#[derive(Default, Debug, Deserialize)]
pub struct Clouds {
    all: i16,
}

#[derive(Default, Debug, Deserialize)]
pub struct Sys {
    // pub type: i16,
    pub id: i32,
    pub country: String,
    pub sunrise: i32,
    pub sunset: i32,
}
