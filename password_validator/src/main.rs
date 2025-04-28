use regex::Regex;
use std::env;
use std::process::exit;

enum Status {
    ERROR
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(value) = args.get(1){
        let result = validate_password(value);

        match result {
            Ok(value) => {
                if value.to_string() == "Medium" {
                    println!("Medium")
                } else {
                    println!("Strong")
                }
            }
            Err(_) => {
                println!("Password need to have 8 chars, uppercase, numbers, lowercase and symbols.");
                exit(1);
            }
        }
    } else {
        println!("Usage: <password>");
        exit(1);
    }
    //println!("{}", result);
}

fn validate_password(password: &str) -> Result<String, Status> {
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    let has_special = password.chars().any(|c| "!@#$^&*()_+".contains(c));

    let min_length = 8;
    let max_length = 12;

    if password.len() < min_length || !has_lowercase || !has_uppercase || !has_digit || !has_special {
        return Err(Status::ERROR);
    }

    if password.len() <= max_length {
        Ok("Medium".to_string())
    } else {
        Ok("Strong".to_string())
    }
}

fn regex_password_validator(password: &str) -> bool{
    let re = Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*\W).+$").unwrap();

    if re.is_match(password) {
        return true;
    }
    return false;
}
