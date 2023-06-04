use colored::*;
use std::io;

fn main() {
    println!("Enter your weight in kg, {}", "q to quit".blue());

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().eq("q") {
            break;
        } else {
            let weight: f32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!();
                    println!("{}", "Please enter a number".red());
                    continue;
                }
            };

            let mars_weight = calculate_weight_on_mars(weight);

            println!("Weight on Mars: {}g", mars_weight);
        }
    }
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
