// use std::collections::HashMap;
// use clap::Parser;
// use serde::{ Deserialize, Serialize };
// use serde_json::{ Value, Result };
use std::*;
use std::process::exit;
use std::fs::*;
use std::io::Write;
mod api_key;
use api_key::ask_for_api_key;
mod ask_user_to_sign_up;
use ask_user_to_sign_up::ask_if_user_has_account;
// #[derive(Parser, Debug)]
// #[clap(name = "Weather CLI", version = "1.0", author = "Awais Amjad")]
// struct Cli {
//     /// The city to check the weather for
//     #[clap(short, long, default_value = "London")]
//     city: String,
// }

// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {
//     let city = "London";
//     let api_key = "2000dfd2364e06482676260c48fde1c9";
//     let url = format!("https://api.openweathermap.org/data/2.5/weather?appid={}&q={}", api_key, city);

//     // let resp = reqwest::get(url).await?;
//     // let resp_json: Value = serde_json::from_str(&resp.text().await?)?;
//     // println!("{}", resp_json["weather"][0]["description"]);
//     let resp = reqwest::Client::new()
//         .get(url)
//         .send()
//         .await?
//         .json()
//         .await?;
//     println!("{:?}", resp["weather"][0]["description"]);
//     Ok(())
// }

fn main() {
    // let cli = Cli::parse();

    // println!("The city is {}", cli.city);
    // ask_for_api_key();
    // if ask_if_user_has_account().unwrap(){
    //     ask_for_api_key();
    // }
    if let Some(true) = ask_if_user_has_account() {
        ask_for_api_key();
    }
    
}