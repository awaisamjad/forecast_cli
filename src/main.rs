// use std::collections::HashMap;
// use clap::Parser;
// use serde::{ Deserialize, Serialize };
// use serde_json::{ Value, Result };
use std::*;
use std::process::exit;
use std::fs::*;
use std::io::Write;
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
    ask_for_api_key();
}
//~ Gets the api key from the input
fn read_user_input_for_api_key() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");
    user_input.trim().to_string()
}

//~ Validates the length of the api key
fn validate_api_key_length(api_key: &str, expected_len: usize) -> bool {
    api_key.len() == expected_len
}

//~ Quits the program if the user enters "Quit"
fn quit_program(input: &String){

    if input.trim().eq_ignore_ascii_case("Quit"){
        exit(0)
    }
}

//~ Asks the user for the api key
fn ask_for_api_key() {
    const API_KEY_LEN: usize = 32;

    loop {
        println!("Enter API key: ");
        let api_key = read_user_input_for_api_key();
        quit_program(&api_key);
        if !validate_api_key_length(&api_key, API_KEY_LEN) {
            println!(
                "\nAPI key must be 32 digits long\nYour input was {} digits long.\nEnter again",
                api_key.len()
            );
        } else {
        //    write_api_key_file()
        create_txt_file_for_api_key(&api_key);
        exit(0)
        }
    }
}

//~ Creates a txt file for the api key
fn create_txt_file_for_api_key(api_key : &String){
    let mut txt_file = File::create("api_key_txt_file.txt").expect("Failed to create file");
    txt_file.write_all(api_key.as_bytes()).expect("Failed to write file");

    println!("Successfully written file");
}
// fn write_api_key_file(api_key: String){
//     exit(0)
// }