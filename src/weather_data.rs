#[derive(Debug, serde::Deserialize)]
pub struct WeatherData {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: u32,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: u64,
    pub sys: Sys,
    pub timezone: i64,
    pub id: u64,
    pub name: String,
    pub cod: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Debug, serde::Deserialize)]
pub struct Weather {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: u32,
    pub humidity: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Clouds {
    pub all: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Sys {
    #[serde(rename = "type")]
    pub sys_type: u32,
    pub id: u32,
    pub country: String,
    pub sunrise: u64,
    pub sunset: u64,
}