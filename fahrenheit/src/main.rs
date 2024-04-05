use std::io;

// Converts Fahrenheight to Celsius and vice-versa
fn main() {
    println!("This will convert values between Fahrenheight and Celsius.\n");
    accept_input();
}

fn accept_input() {
    println!("Please enter \"f\" for Fahrenheit and \"c\" for Celsius");

    let mut starting_temp = String::new();

    io::stdin()
        .read_line(&mut starting_temp)
        .expect("Cannot read line");

    let starting_temp = match starting_temp.trim() {
        "f" => "f",
        "F" => "f",
        "Fahrenheit" => "f",
        "fahrenheit" => "f",
        "C" => "c",
        "c" => "c",
        "Celsius" => "C",
        "celsius" => "c",
        _ => "n",
    };

    match starting_temp {
        "f" => convert_fahrenheight(),
        "c" => convert_celsius(),
        _ => handle_input_err(),
    }
}

fn convert_fahrenheight() {
    println!("this is the logic for fahrenheight");
}

fn convert_celsius() {
    println!("this is the logic for celsius");
}

fn handle_input_err() {
    println!("Invalid entry. You must specify Celsius(c) or Fahrenheight(f). Please try again.\n");
    accept_input();
}
