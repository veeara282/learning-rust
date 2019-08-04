use std::io;
use std::f32::NAN;

fn main() {
    let degrees: f32 = read_float();
    let unit: char = read_unit();
    println!(
        "The temperature you entered is {}",
        format_temperature(degrees, unit)
    );
    warn_absolute_zero(degrees, unit);
    println!(
        "{} = {}",
        format_temperature(degrees, unit),
        format_temperature(
            match unit {
                'F' => to_celsius(degrees),
                'C' => to_fahrenheit(degrees),
                _ => NAN
            },
            match unit {
                'F' => 'C',
                'C' => 'F',
                _ => '\0'
            }
        )
    );
}

fn to_celsius(f: f32) -> f32 {
    (f + 40.0) / 1.8 - 40.0
}

fn to_fahrenheit(c: f32) -> f32 {
    (c + 40.0) * 1.8 - 40.0
}

fn warn_absolute_zero(degrees: f32, unit: char) {
    let abs_zero = match unit {
        'F' => -459.67,
        'C' => -273.15,
        _ => NAN
    };
    if degrees < abs_zero {
        println!(
            "Warning! {} is less than absolute zero ({}).",
            format_temperature(degrees, unit),
            format_temperature(abs_zero, unit)
        );
    }
}

fn format_temperature(degrees: f32, unit: char) -> String {
    format!("{} °{}", degrees, unit)
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

fn read_unit() -> char {
    loop {
        println!("Enter a symbol (C or F): ");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        match line.trim().to_ascii_uppercase().as_ref() {
            "F" => break 'F',
            "C" => break 'C',
            unit => {
                println!("Invalid unit: {}", unit);
                continue;
            }
        }
    }
}
