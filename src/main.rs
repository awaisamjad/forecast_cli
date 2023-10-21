// use std::collections::HashMap;
use clap::Parser;
use lazy_static::lazy_static;
use serde_json::*;
use iso_country::*;
mod api_key;
mod ask_user_to_sign_up;
use std::collections::HashMap;
use std::fmt::format;
use std::io;
mod get_weather_data;
// use get_weather_data::get_api_from_txt_file;

//~ Makes a global static variable for the default city value used in the city (CLI) argument
lazy_static! {
    static ref DEFAULT_CITY_VALUE: String = "London".to_string();
}
#[derive(Parser, Debug)]
#[clap(name = "Weather CLI", version = "1.0", author = "Awais Amjad")]
struct Cli {
    /// The city to check the weather for
    #[clap(short, long, default_value = DEFAULT_CITY_VALUE.as_str())]
    city: String,
}

// fn main() {
//     let mut api_key_path = String::new();
//     // let cli = Cli::parse();

//     // println!("The city is {}", cli.city);
//     // ask_for_api_key();
//     // if ask_if_user_has_account().unwrap(){
//     //     ask_for_api_key();
//     // }
//     // if api_key_exists() {
//     //     println!("api key exists");

//     //     println!("Running App")
//     // }
//     // else {
//     //     println!("api key doesnt exist");
//     // }

//     // //~ if user has account, ask for api key
//     if let Some(true) = ask_if_user_has_account() {
//         api_key_path = ask_for_api_key();
//         println!("{}", api_key_path);
//     }

//     // if_api_key_exists();
//     // let mut file_destination = String::new();

//     // io::stdin()
//     //     .read_line(&mut file_destination)
//     //     .expect("Failed to read input");

//     // file_destination = file_destination.trim().to_string();

//     // let mut path = format!("{}/api_key.txt", file_destination);
//     // path = Path::new(&path).to_str().unwrap().to_string();

//     // println!("{}", path);

//     // pub fn get_api_from_txt_file()  {
//     //     let file_path = create_txt_file_for_api_key();
//     //     // let file_path = "C:/Users/User/Code/Rust/forecast_cli/api_key_file.txt";
//     //     let mut file = fs::read_to_string(file_path).expect("Unable to open file");

//     //     println!("{}",file );
//     // }
//     // get_api_from_txt_file();
// }
use error_chain::error_chain;
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}
#[derive(Debug, serde::Deserialize)]
struct WeatherData {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    visibility: u32,
    wind: Wind,
    clouds: Clouds,
    dt: u64,
    sys: Sys,
    timezone: i64,
    id: u64,
    name: String,
    cod: u32,
}

#[derive(Debug, serde::Deserialize)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Debug, serde::Deserialize)]
struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, serde::Deserialize)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: u32,
    humidity: u32,
}

#[derive(Debug, serde::Deserialize)]
struct Wind {
    speed: f64,
    deg: u32,
}

#[derive(Debug, serde::Deserialize)]
struct Clouds {
    all: u32,
}

#[derive(Debug, serde::Deserialize)]
struct Sys {
    #[serde(rename = "type")]
    sys_type: u32,
    id: u32,
    country: String,
    sunrise: u64,
    sunset: u64,
}
fn main() -> Result<()> {
    let cli = Cli::parse();
    // let mut city = String::new();
    // io::stdin()
    //     .read_line(&mut city)
    //     .expect("Failed to read City");
    let city = cli.city;
    let api_key = "2000dfd2364e06482676260c48fde1c9";
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?appid={}&q={}",
        api_key,
        city
    );
    let resp = reqwest::blocking::get(url)?;
    let body: WeatherData = resp.json()?;

    // println!("{:#?}",body.weather[0].id);
    default_output(body);
    
    Ok(())
}

fn degrees_celsius(value: f64) -> f64 {
    let rounded_value = (value - 273.15).round() * 100.0;
    rounded_value / 100.0
}

fn default_output(body: WeatherData) {
    let weather_main = &body.weather[0].main;
    let weather_description = &body.weather[0].description;

    let weather = format!(
        "Weather:\n  Main: {}\n  Description: {}",
        weather_main,
        weather_description
    );

    let temp = format!("Temperature: {}", degrees_celsius(body.main.temp));
    let feels_like = format!("Feels Like: {}", degrees_celsius(body.main.feels_like));
    let temp_min = format!("Minimum Temperature: {}", degrees_celsius(body.main.temp_min));
    let temp_max = format!("Maximum Temperature: {}", degrees_celsius(body.main.temp_max));
    let pressure = format!("Pressure: {}", body.main.pressure);
    let humidity = format!("Humidity: {}", body.main.humidity);

    let main = format!(
        "\nMain:\n {}\n {}\n {}\n {}\n {}\n {}",
        temp,
        feels_like,
        temp_min,
        temp_max,
        pressure,
        humidity
    );

    let visibility = format!("\nVisibility: {}", body.visibility);

    let wind_speed = format!("Wind Speed: {}", body.wind.speed);
    let wind_deg = format!("Wind Degree: {}", body.wind.deg);

    let wind = format!("\nWind:\n {}\n {}", wind_speed, wind_deg);

    let clouds = format!("\nClouds: {}", body.clouds.all);

    let sys_country = format!("Country: {}", body.sys.country);
    let full_country_name = format!("Full Country Name: {}", get_full_country_name_from_small(&body.sys.country)); //~ has a reference which converts String to &str
    let sys_sunrise = format!("Sunrise: {}", body.sys.sunrise);
    let sys_sunset = format!("Sunset: {}", body.sys.sunset);

    let sys = format!("\nSys:\n {}\n {}\n {}\n {}", sys_country, full_country_name, sys_sunrise, sys_sunset);

    let timezone = format!("\nTimezone: {}", body.timezone);
    let name = format!("\nID: {}", body.name);

    let output = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        weather,
        main,
        visibility,
        wind,
        clouds,
        sys,
        timezone,
        name
    );

    println!("{}", output)
}

fn get_full_country_name_from_small(small_name:&str) -> &str{
    let mut countries = iso_country::data::all();
    countries.sort_by_key(|country| country.alpha2);
    let mut countries_hashmap = HashMap::new();
    for country in countries {
        countries_hashmap.insert( country.alpha2, country.name);
    }
    let full_country_name = countries_hashmap.get(&small_name);
    if let Some(value) = full_country_name{
        *value //~ the asterik derefrences it - goes from &&str to &str
    } else{
        "Error"
    }
}//Option<&str>