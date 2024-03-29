# Temperature Converter

A simple command-line tool to convert temperatures between Celsius and Fahrenheit.

## Installation

Ensure you have Rust and Cargo installed on your system. Clone this repository and navigate into the project directory:

```
git clone https://github.com/KnollElias/TemperatureConverter.git
cd TemperatureConverter
```

Build the project using Cargo:

```
cargo build --release
```

The executable will be located in `./target/release/`.

## Usage

To convert a temperature from Celsius to Fahrenheit:

```
tempconv --convFahr `input_temperature`
```

To convert a temperature from Fahrenheit to Celsius:

```
tempconv --convCels `input_temperature`
```

Replace `input_temperature` with the temperature you wish to convert.

## Examples

Convert 20°C to Fahrenheit:

```
tempconv --convFahr 20
```

Convert 68°F to Celsius:

```
tempconv --convCels 68
```
