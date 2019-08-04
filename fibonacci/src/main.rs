use std::io;

fn main() {
    let input = read_int();
    println!("You entered: {}", input);

    let output = fibonacci(input);
    println!("The {} Fibonacci number is {}", ordinal(input), output)
}

fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let mut values = [0, 1];
    for i in 2..n+1 {
        let index = (i % 2) as usize;
        values[index] = values[0] + values[1];
    }
    values[(n % 2) as usize]
}

fn ordinal(i: u64) -> String {
    match i {
        1 => "1st".to_string(),
        2 => "2nd".to_string(),
        3 => "3rd".to_string(),
        _ => format!("{}th", i),
    }
}

fn read_int() -> u64 {
    loop {
        println!("Enter a number less than 94:");
        
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        
        match line.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please type a non-negative integer.");
                continue;
            }
        }
    }
}
