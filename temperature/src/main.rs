use crate::Unit::*;
use std::io;

fn main() {
    let degrees: f32 = read_float();
    let unit: Unit = read_unit();
    let temperature = Temperature::new(degrees, unit);
    println!("The temperature you entered is {}", temperature.to_string());
    warn_absolute_zero(temperature);
    println!(
        "{} = {}",
        temperature.to_string(),
        convert(temperature).to_string()
    );
}

fn convert(t: Temperature) -> Temperature {
    match t.unit {
        Fahrenheit => Temperature::new(to_celsius(t.degrees), Celsius),
        Celsius => Temperature::new(to_fahrenheit(t.degrees), Fahrenheit),
    }
}

#[derive(Copy, Clone)]
enum Unit {
    Celsius,
    Fahrenheit,
}

impl ToString for Unit {
    fn to_string(&self) -> String {
        match self {
            Fahrenheit => "°F".to_string(),
            Celsius => "°C".to_string(),
        }
    }
}

#[derive(Copy, Clone)]
struct Temperature {
    degrees: f32,
    unit: Unit,
}

impl Temperature {
    fn new(degrees: f32, unit: Unit) -> Temperature {
        Temperature {
            degrees: degrees,
            unit: unit,
        }
    }
}

impl ToString for Temperature {
    fn to_string(&self) -> String {
        format!("{} {}", self.degrees, self.unit.to_string())
    }
}

fn to_celsius(f: f32) -> f32 {
    (f + 40.0) / 1.8 - 40.0
}

fn to_fahrenheit(c: f32) -> f32 {
    (c + 40.0) * 1.8 - 40.0
}

fn warn_absolute_zero(t: Temperature) {
    let abs_zero = match t.unit {
        Fahrenheit => Temperature::new(-459.67, Fahrenheit),
        Celsius => Temperature::new(-273.15, Celsius),
    };
    if t.degrees < abs_zero.degrees {
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
