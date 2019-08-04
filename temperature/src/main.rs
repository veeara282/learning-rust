use std::io;
use crate::Unit::*;

fn main() {
    let degrees: f32 = read_float();
    let unit: Unit = read_unit();
    let temperature = Temperature(degrees, unit);
    println!(
        "The temperature you entered is {}",
        temperature.to_string()
    );
    warn_absolute_zero(temperature);
    println!(
        "{} = {}",
        temperature.to_string(),
        convert(temperature).to_string()
    );
}

fn convert(t: Temperature) -> Temperature {
    match t.1 {
        Fahrenheit => Temperature(to_celsius(t.0), Celsius),
        Celsius => Temperature(to_fahrenheit(t.0), Fahrenheit)
    }
}

#[derive(Copy, Clone)]
enum Unit {
    Celsius,
    Fahrenheit
}

impl ToString for Unit {
    fn to_string(&self) -> String {
        match self {
            Fahrenheit => "°F".to_string(),
            Celsius => "°C".to_string()
        }
    }
}

#[derive(Copy, Clone)]
struct Temperature(f32, Unit);

impl ToString for Temperature {
    fn to_string(&self) -> String {
        format!("{} {}", self.0, self.1.to_string())
    }
}

fn to_celsius(f: f32) -> f32 {
    (f + 40.0) / 1.8 - 40.0
}

fn to_fahrenheit(c: f32) -> f32 {
    (c + 40.0) * 1.8 - 40.0
}

fn warn_absolute_zero(t: Temperature) {
    let abs_zero = match t.1 {
        Fahrenheit => Temperature(-459.67, Fahrenheit),
        Celsius => Temperature(-273.15, Celsius)
    };
    if t.0 < abs_zero.0 {
        println!(
            "Warning! {} is less than absolute zero ({}).",
            t.to_string(),
            abs_zero.to_string()
        );
    }
}

fn read_float() -> f32 {
    loop {
        println!("Enter a number:");
        
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        
        match line.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        }
    }
}

fn read_unit() -> Unit {
    loop {
        println!("Enter a symbol (C or F): ");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        match line.trim().to_ascii_uppercase().as_ref() {
            "F" => break Fahrenheit,
            "C" => break Celsius,
            unit => {
                println!("Invalid unit: {}", unit);
                continue;
            }
        }
    }
}
