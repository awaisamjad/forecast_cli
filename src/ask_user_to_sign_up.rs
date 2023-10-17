use std::io;
pub fn ask_if_user_has_account() -> Option<bool> {
    println!("This program uses the OpenWeatherMap API to get weather data");
    println!("Do you have an account at https://openweathermap.org ? (y/n)");

    let mut user_has_account = String::new();
    io::stdin()
        .read_line(&mut user_has_account)
        .expect("Failed to read line");

    let user_has_account = user_has_account.trim();
    
    if user_has_account == "y" {
        return Some(true);
    } 
    else if user_has_account == "n" {
        println!("Please create an account at https://openweathermap.org");
        return Some(false);
    } 
    else {
        //~ If input isnt y or n, ask again
        println!("Please enter y or n");
        ask_if_user_has_account();
        None
    }
}
