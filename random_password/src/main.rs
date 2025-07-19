use clap::Parser;
use rand::Rng;
use std::{
    env,
    io::{self, Write},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "8", help = "Set password length")]
    length: String,

    #[arg(short, long, help = "Insert simple letters only")]
    letters: bool,

    #[arg(short, long, help = "Insert accent letters")]
    accent: bool,

    #[arg(short, long, help = "Insert letters and numbers")]
    numbers: bool,

    #[arg(short, long, help = "Insert special symbols")]
    symbols: bool,

    #[arg(
        short,
        long,
        default_value = "abc",
        help = "Limit characters to the specified ones"
    )]
    custom: String,
}

fn get_password_length(args: &Args, has_options: bool) -> String {
    if has_options {
        args.length.clone()
    } else {
        let mut length = String::new();
        print!("Insert the password length: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut length)
            .expect("Failed to get length");

        length
    }
}

fn standard_password(length: u32) -> String {
    let mut password = String::new();

    for _ in 0..length {
        let random: u8 = rand::thread_rng().gen_range(33..=122);
        let character = random as char;
        password.push(character);
    }

    password
}

fn option_password(args: &Args, length: u32) -> String {
    let letters = args.letters;
    let accent = args.accent;
    let numbers = args.numbers;
    let symbols = args.symbols;
    let custom = args.custom.clone();

    if letters {
        letters_password(length).0
    } else if numbers || symbols {
        mix_password(length, accent, numbers, symbols)
    } else if !custom.is_empty() {
        custom_password(length, custom)
    } else {
        standard_password(length)
    }
}

fn letters_password(length: u32) -> (String, Vec<u8>) {
    let mut password = String::new();
    let mut ascii = vec![];

    for _ in 0..length {
        loop {
            let random: u8 = rand::thread_rng().gen_range(65..=122);

            if (random > 64 && random < 91) || (random > 96 && random < 123) {
                password.push(random as char);
                ascii.push(random);
                break;
            }
        }
    }

    (password, ascii)
}

fn mix_password(length: u32, accent: bool, numbers: bool, symbols: bool) -> String {
    let mut password = String::new();
    let mut chars: Vec<u8> = vec![];

    for n in letters_password(length).1 {
        chars.push(n);
    }

    if accent {
        chars.push(159);

        for n in 192..=255 {
            if n != 215 && n != 222 && n != 247 && n != 254 {
                chars.push(n);
            }
        }
    }

    if numbers {
        for n in 48..=57 {
            chars.push(n);
        }
    }

    if symbols {
        for n in 33..=167 {
            if (n > 32 && n < 48)
                || (n > 57 && n < 65)
                || (n > 90 && n < 97)
                || (n > 122 && n < 126)
            {
                chars.push(n);
            }
        }
    }

    for _ in 0..length {
        let random: u8 = rand::thread_rng().gen_range(0..chars.len() as u8);
        password.push(chars[random as usize] as char);
    }

    password
}

fn custom_password(length: u32, custom: String) -> String {
    let mut password = String::new();
    let charset: Vec<char> = custom.chars().collect();

    for _ in 0..length {
        let random = rand::thread_rng().gen_range(0..charset.len());
        password.push(charset[random]);
    }

    password
}

fn main() {
    let args = Args::parse();
    let has_options = env::args().len() > 1;
    let length = get_password_length(&args, has_options);

    let length: u32 = length.trim().parse().expect("Not a number!");
    let password = String::from(if has_options {
        standard_password(length)
    } else {
        option_password(&args, length)
    });

    println!("Your new password is: {password}");
}
