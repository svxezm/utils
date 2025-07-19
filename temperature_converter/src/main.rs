use std::io::{self, Write};

fn main() {
    print!("Enter a quantity: ");
    let mut temperature_input = String::new();
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut temperature_input).unwrap();
    let temperature: f32 = temperature_input
        .trim()
        .parse()
        .expect("Please, insert only numbers for temperature.");

    print!("Enter the unit: ");
    let mut unit_input = String::new();
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut unit_input).unwrap();

    let mut celsius: f32 = 0.0;
    let mut fahrenheit: f32 = 0.0;
    let mut kelvin: f32 = 0.0;

    match unit_input.to_lowercase().chars().next() {
        Some('c') => {
            celsius = temperature;
            fahrenheit = (1.8 * celsius) + 32.0;
            kelvin = celsius + 273.15;
        }
        Some('f') => {
            fahrenheit = temperature;
            celsius = ((fahrenheit - 32.0) * 5.0) / 9.0;
            kelvin = ((fahrenheit - 32.0) * 5.0) / 9.0 + 273.15;
        }
        Some('k') => {
            kelvin = temperature;
            celsius = temperature - 273.15;
            fahrenheit = ((kelvin - 273.15) * 9.0) / 5.0 + 32.0;
        }
        _ => println!("Unit not available."),
    }

    println!("Results:\n");
    println!("C: {:.1}", celsius);
    println!("F: {:.1}", fahrenheit);
    println!("K: {:.1}", kelvin);
}
