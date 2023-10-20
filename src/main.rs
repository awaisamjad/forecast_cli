// use std::collections::HashMap;
// use clap::Parser;
use serde::{ Deserialize, Serialize };
use serde_json::*;
mod api_key;
use api_key::{ ask_for_api_key, if_api_key_exists, create_txt_file_for_api_key };
mod ask_user_to_sign_up;
use ask_user_to_sign_up::ask_if_user_has_account;
use std::fs::{ File, self };
use std::io::{ self, Read };
use std::path::Path;
mod get_weather_data;
// use get_weather_data::get_api_from_txt_file;
// #[derive(Parser, Debug)]
// #[clap(name = "Weather CLI", version = "1.0", author = "Awais Amjad")]
// struct Cli {
//     /// The city to check the weather for
//     #[clap(short, long, default_value = "London")]
//     city: String,
// }



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
    let city = "London";
    let api_key = "2000dfd2364e06482676260c48fde1c9";
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?appid={}&q={}",
        api_key,
        city
    );
    let resp = reqwest::blocking::get(url)?;
    let body : Value = resp.json()?;
    println!("{:#?}", body);

    Ok(())
}
