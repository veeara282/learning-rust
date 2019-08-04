use std::io;

fn main() {
    let input = read_int();
    println!("You entered: {}", input);
}

fn read_int() -> u32 {
    loop {
        println!("Enter a number:");
        
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
