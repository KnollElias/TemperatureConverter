use clap::{App, Arg};

pub fn run() {
    let matches = App::new("tempconv")
        .version("1.0")
        .author("Your Name")
        .about("Converts temperatures between Fahrenheit and Celsius")
        .arg(
            Arg::with_name("convFahr")
                .long("convFahr")
                .value_name("TEMPERATURE")
                .help("Converts a Celsius temperature to Fahrenheit")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("convCels")
                .long("convCels")
                .value_name("TEMPERATURE")
                .help("Converts a Fahrenheit temperature to Celsius")
                .takes_value(true),
        )
        .get_matches();

    if let Some(c) = matches.value_of("convCels") {
        match c.parse::<f32>() {
            Ok(temp) => println!(
                "{}°C is {}°F",
                temp,
                crate::transform::convertToFahrenheit(temp)
            ),
            Err(_) => println!("Invalid input temperature"),
        }
    } else if let Some(f) = matches.value_of("convFahr") {
        match f.parse::<f32>() {
            Ok(temp) => println!(
                "{}°F is {}°C",
                temp,
                crate::transform::convertToCelsius(temp)
            ),
            Err(_) => println!("Invalid input temperature"),
        }
    } else {
        println!("Invalid input temperature");
    }
}
