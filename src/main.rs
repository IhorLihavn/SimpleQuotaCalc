use std::io::{self, Write};

fn main() {
    
    let soled_scrap = input_to_int("Soled scrap: ");
    let quota = input_to_int("Quota: ");

    let res = (soled_scrap-quota)/5 - 15;
    println!("Your bonus is {}", res);
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn input_to_int(prompt: &str) -> u32 {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<u32>() {
        Ok(e) => {
            e
        },
        Err(e) => {
            println!("ERROR: please write a {} number:", prompt);
            input_to_int(prompt)
        }
    }
}