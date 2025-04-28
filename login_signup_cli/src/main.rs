use core::panic;
use std::io::{self, Write};
use std::env;
use rpassword::read_password;
use regex::Regex;

enum Status {
    SUCCESS,
    ERROR,
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(value) = args.get(1) {
        match value.as_str() {
            "login" => login(),
            "signup" => signup(),
            _ => panic!("Error: Invalid option!")
        }
    } else {
        panic!("Error: No option provided!");
    }
}

fn signup(){
    let email= read_input("Email");
    match validate_email(&email) {
        Ok(_) => println!("Choose your password."),
        Err(err) => {
            println!("{}", err);
            return;
        }
    }

    std::io::stdout().flush().expect("Error cleaning buffer.");
    let password = read_password().expect("Failed to read password.");

    match validate_password(&password) {
        Ok(_) => println!("Hi {}, your account has been created!", email),
        Err(err) => {
            println!("{}", err);
            return;
        }
    }
}

fn login(){
    let password = prompt_password("mypwd");
    match password {
        Status::SUCCESS => println!("Logged sucessfully."),
        Status::ERROR => println!("Error logging in.")
    }
}

#[allow(dead_code)]
fn prompt_password(target_value: &str) -> Status{
    const MAX_ATTEMPTS: u8 = 3;
    let mut attempts: u8 = 0;
    let result = loop {
        print!("Password: ");
        std::io::stdout().flush().expect("Error cleaning buffer.");
        let input: String = read_password().expect("Falha ao ler a senha");

        if input == target_value {
            break Status::SUCCESS;
        }

        if attempts == MAX_ATTEMPTS {
            break Status::ERROR;
        }
        println!("Wrong password. Try again.");
        attempts += 1;
    };
    result
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();

    print!("{}: ", prompt);
    io::stdout().flush().expect("Failed cleaning buffer");

    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");

    input.trim().to_string()
}

fn validate_email(email: &str) -> Result<(), String> {
    let email_regex = match Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$") {
        Ok(regex) => regex,
        Err(_) => return Err("Invalid regex pattern".to_string()),
    };

    let is_valid_email = email_regex.is_match(email);
    if !is_valid_email {
        return Err("Invalid email!".to_string());
    } else {
        Ok(())
    }
}

fn validate_password(password: &str) -> Result<(), String> {
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    let has_special = password.chars().any(|c| "!@#$^&*()_+".contains(c));

    let min_length  = 8;
    if password.len() < min_length || !has_lowercase || !has_uppercase || !has_digit || !has_special {
        return Err("Invalid password. Must have uppercase, lowercase letters, symbols and numbers.".to_string());
    } else {
        return Ok(());
    }
}