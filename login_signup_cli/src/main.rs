use core::panic;
use std::io::{self, Write};
use std::env;
use rpassword::read_password;
use regex::Regex;

//let mut my_vec = vec![56, 82, 12, 23, 15, 99, 51, 35, 41, 27];
//remove_value_from_array2(&mut my_vec, 35);
//let value_exists = search_for_value(&mut my_vec, 15);

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
        Ok(_) => {},
        Err(err) => {
            println!("{}", err);
            return;
        }
    }

    print!("Password: ");
    std::io::stdout().flush().expect("Error cleaning buffer.");
    let password = read_password().expect("Falha ao ler a senha");
    validate_password(password.as_str());
    println!("Hi {}, your account has been created!", email);
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
    io::stdout().flush().expect("Falha ao limpar o buffer de saída");

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

#[allow(dead_code)]
fn validate_password(password: &str){
    print!("{}", password);
}
