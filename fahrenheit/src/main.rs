use std::io;

// Converts Fahrenheight to Celsius and vice-versa
fn main() {
    println!("This will convert values between Fahrenheight and Celsius.\n");
    let converted = prompt_measurement().to_string().to_owned();
    let s = String::from(converted);
    println!("{s}")
}

fn prompt_measurement() -> f64 {
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
        "f" => convert_to_celsius(),
        "c" => convert_to_fahrenheight(),
        _ => prompt_measurement(),
    }
}

fn get_temp() -> f64 {
    println!("Please enter a temperature to convert.");
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Unable to read line");

    println!("the value of temp is {temp}");

    temp.trim().parse().unwrap()
}

fn convert_to_celsius() -> f64 {
    const CONVERSION_COEFFICIENT: f64 = 5_f64 / 9_f64;
    const CONVERSION_CONSTANT: f64 = -32.0;

    let temp = get_temp();
    return (temp + CONVERSION_CONSTANT) * CONVERSION_COEFFICIENT;
}

fn convert_to_fahrenheight() -> f64 {
    const CONVERSION_COEFFICIENT: f64 = 9_f64 / 5_f64;
    const CONVERSION_CONSTANT: f64 = 32.0;

    let temp = get_temp();
    return temp * CONVERSION_COEFFICIENT + CONVERSION_CONSTANT;
}
