use reqwest;
use serde::Deserialize;
use std::{collections::HashMap, io::Write};

#[derive(Debug, Deserialize)]
#[serde(tag = "result")]
enum ApiResponse {
    #[serde(rename = "success")]
    Success { rates: HashMap<String, f32> },
    #[serde(rename = "error")]
    Error {
        #[serde(rename = "error-type")]
        error_type: String,
    },
}

fn print_comparison(base: &str, secondary: &str, quantity: f32, currencies: &HashMap<String, f32>) {
    match currencies.get(secondary) {
        Some(rate) => {
            println!("\n{: >5}  {: >5}", base, secondary);

            let total_amount = quantity * rate;
            println!("{: >5.2} = {: >5.2}", quantity, total_amount);

            if quantity != 1.0 {
                println!("\nConversion rate: 1 {} -> {:.2} {}", base, rate, secondary);
            }
        }
        None => eprintln!("Currency not found"),
    }
}

fn print_full_list(currencies: &HashMap<String, f32>) {
    let mut entries: Vec<(&String, &f32)> = currencies.iter().collect();
    entries.sort_by_key(|&(code, _)| code);

    println!("Rates:");
    for (code, value) in entries {
        println!("{:<5} -> {}", code, value);
    }
}

fn prompt_quantity(base: &str, secondary: &str, rates: &HashMap<String, f32>) {
    loop {
        let mut quantity = String::new();
        print!("How much? ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut quantity)
            .expect("Quantity currency not informed");
        let quantity: f32 = if !quantity.trim().is_empty() {
            quantity.trim().parse().expect("Quantity was not a number")
        } else {
            1.0
        };

        if !secondary.is_empty() && secondary != "ALL" {
            print_comparison(&base, &secondary, quantity, &rates);
        } else {
            print_full_list(&rates);
        };

        let mut repeat = String::new();
        print!("\n\nEnter new amount? [y, N] ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut repeat).unwrap();

        match repeat.trim() {
            "y" | "Y" => println!("\n============\n"),
            _ => break,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut base = String::new();
    let mut secondary = String::new();

    print!("Insert the base currency: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut base)
        .expect("Base currency not informed");
    base = base.trim().to_uppercase();

    let url = format!("https://open.er-api.com/v6/latest/{}", base);
    let response = reqwest::get(&url).await?;

    let response_json: ApiResponse = response.json().await?;
    let rates = match response_json {
        ApiResponse::Success { rates } => rates,
        ApiResponse::Error { error_type } => {
            eprintln!(
                "Oopsieee, '{}' is not a valid code :p\nAPI said: '{}'",
                base, error_type
            );
            std::process::exit(1);
        }
    };

    print!("Which currency to compare? ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut secondary).unwrap();
    secondary = secondary.trim().to_uppercase();

    prompt_quantity(&base, &secondary, &rates);

    Ok(())
}
