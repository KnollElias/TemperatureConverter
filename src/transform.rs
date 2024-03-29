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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_celsius() {
        assert_eq!(convertToCelsius(32.0), 0.0);
        assert_eq!(convertToCelsius(212.0), 100.0);
        assert_eq!(convertToCelsius(INPUT_TEMPERATURE_VAR), -6.36);
    }

    #[test]
    fn test_convert_to_fahrenheit() {
        assert_eq!(convertToFahrenheit(0.0), 32.0);
        assert_eq!(convertToFahrenheit(100.0), 212.0);
        assert_eq!(convertToFahrenheit(-6.36), 20.55);
    }

    #[test]
    fn test_round_temperature() {
        assert_eq!(roundTemeperature(32.75111), 32.75);
        assert_eq!(roundTemeperature(32.259), 32.26);
    }
}
