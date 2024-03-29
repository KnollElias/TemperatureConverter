pub const INPUT_TEMPERATURE_VAR: f32 = 20.5556;

fn roundTemeperature(input_temerature: f32) -> f32 {
    let rounded = (input_temerature * 100.0).round() / 100.0;
    rounded
}

pub fn convertToCelsius(input_temperature: f32) -> f32 {
    let celsius = (input_temperature.clone() - 32.0) * 5.0 / 9.0;
    let celsius_rounded = roundTemeperature(celsius.clone());
    celsius_rounded
}

pub fn convertToFahrenheit(input_temerature: f32) -> f32 {
    let fahrenheit = (input_temerature.clone() * 9.0 / 5.0) + 32.0;
    let fahrenheit_rounded = roundTemeperature(fahrenheit.clone());
    fahrenheit_rounded
}
