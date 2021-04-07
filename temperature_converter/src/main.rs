use std::io;
use std::io::Write;

fn main() {
    println!("TEMPERATURE CONVERTER");
    println!("----------------------------------");
    println!("1. C to F");
    println!("2. F to C");
    println!("Please choose a conversion type:");

    loop {
        let mut conversion_type: String = String::new();
        io::stdin()
            .read_line(&mut conversion_type)
            .expect("Failed to read line");
        let conversion_type: u8 = match conversion_type.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };
        if conversion_type > 2 || conversion_type < 1 {
            println!("Please enter a 1 or 2");
            continue;
        }
        print!("Please enter temperature value: ");
        io::stdout().flush().expect("Failed to print");
        loop {
            let mut value = String::new();
            io::stdin()
                .read_line(&mut value)
                .expect("Failed to read line");
            let value: f64 = match value.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number");
                    continue;
                }
            };
            if conversion_type == 1 {
                let result: f64 = (value * (9.0 / 5.0)) + 32.0;
                println!("==>{:.2}C = {:.2}F", value, result);
                break;
            } else if conversion_type == 2 {
                let result: f64 = (value - 32.0) * (5.0 / 9.0);
                println!("==>{:.2}F = {:.2}C", value, result);
                break;
            }
        }
        break;
    }
}
