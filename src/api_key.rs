use std::*;
use std::process::exit;
use std::fs::*;
use std::io::Write;
use std::path::Path;
//~ Check if api key exists
pub fn if_api_key_exists() -> bool {
    let path_to_check = "C:/Users/User/Code/Rust/forecast_cli";

    let path = Path::new(path_to_check);

    if path.exists() {
        println!("The path exists: {:?}", path);
        true
    } else {
        println!("The path does not exist: {:?}", path);
        false
    }
}

//~ Gets the api key from the input
pub fn read_user_input_for_api_key() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");
    user_input.trim().to_string()
}

//~ Validates the length of the api key
pub fn validate_api_key_length(api_key: &str, expected_len: usize) -> bool {
    api_key.len() == expected_len
}

//~ Quits the program if the user enters "Quit"
pub fn quit_program(input: &String){

    if input.trim().eq_ignore_ascii_case("Quit"){
        exit(0)
    }
}

//~ Asks the user for the api key
pub fn ask_for_api_key() -> String {
    const API_KEY_LEN: usize = 32;

    loop {
        println!("Enter API key: ");
        let api_key = read_user_input_for_api_key();
        quit_program(&api_key);
        if !validate_api_key_length(&api_key, API_KEY_LEN) {
            println!(
                "\nAPI key must be 32 digits long\nYour input was {} digits long.\nEnter again\n",
                api_key.len()
            );
        } else {
        //    write_api_key_file()
        let api_key_path = create_txt_file_for_api_key(&api_key);
        return api_key_path;
        }
    }
}
//~ prompt for path of the api key file
pub fn prompt_for_api_key_file_path() -> String {
    println!("Enter path for api key file: ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");
    user_input.trim().to_string()
}

//~ Creates a txt file for the api key
pub fn create_txt_file_for_api_key(api_key : &String)-> String {
    let path = prompt_for_api_key_file_path();
    let new_path = format!("{}/api_key_file.txt", path).to_string();
    let mut txt_file = File::create(&new_path).expect("Failed to create file");
    txt_file.write_all(api_key.as_bytes()).expect("Failed to write file");

    println!("Successfully written file to path: {} ", &new_path);
    new_path
}

