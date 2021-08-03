use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut input = String::new();
    handle_input(&mut input)
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

fn handle_input(input: &mut String) {
    let stdin = io::stdin();
    match stdin.read_line(input) {
        Ok(_n) => {
            match input.trim().parse::<f32>() {
                Ok(result) => { 
                    let mars_weight = calculate_weight_on_mars(result);   
                    println!("Weight on Mars: {}kg", mars_weight);
                }
                Err(error) => println!("error: {}", error)
            };
        }
        Err(error) => println!("error: {}", error),
    }
}