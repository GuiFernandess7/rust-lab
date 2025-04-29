use core::panic;
use std::collections::HashMap;
use std::io::{self, Write};
use std::env;
use rpassword::read_password;
use std::fs;
use regex::Regex;
use bcrypt::{hash, DEFAULT_COST, verify};

pub struct Auth {
    email: String,
    password: String,
}

impl Auth {
    pub fn validate_email(&self) -> Result<(), String> {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .map_err(|_| "Invalid regex pattern".to_string())?;

        if !email_regex.is_match(&self.email) {
            Err("Invalid email!".to_string())
        } else {
            Ok(())
        }
    }

    pub fn validate_password(&self) -> Result<(), String> {
        let has_lowercase = self.password.chars().any(|c| c.is_lowercase());
        let has_uppercase = self.password.chars().any(|c| c.is_uppercase());
        let has_digit = self.password.chars().any(|c| c.is_digit(10));
        let has_special = self.password.chars().any(|c| c.is_ascii_punctuation());

        let min_length = 8;
        if self.password.len() < min_length
            || !has_lowercase
            || !has_uppercase
            || !has_digit
            || !has_special
        {
            return Err("Invalid password. Must have uppercase, lowercase letters, symbols and numbers.".to_string());
        }

        Ok(())
    }

    pub fn verify_password(&self, stored_hash: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(self.password.as_str(), stored_hash)
    }

    pub fn hash_password(&self) -> Result<String, bcrypt::BcryptError> {
        hash(self.password.as_str(), DEFAULT_COST)
    }
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

fn signup() -> (){
    let email= read_input("Email");

    let mut auth: Auth = Auth { email: email.clone(), password: String::new() };

    match auth.validate_email() {
        Ok(_) => println!("Choose your password."),
        Err(err) => {
            println!("{}", err);
            return;
        }
    }

    std::io::stdout().flush().expect("Error cleaning buffer.");
    let password = read_password().expect("Failed to read password.");
    auth.password = password.clone();

    match auth.validate_password() {
        Ok(_) => println!("Hi {}, your account has been created!", email),
        Err(err) => {
            println!("{}", err);
            return;
        }
    }

    match auth.hash_password() {
        Ok(hashed_password) => {
            let mut users: Vec<HashMap<String, &str>> = vec![];
            let mut user: HashMap<String, &str> = HashMap::new();
            user.insert(email, hashed_password.as_str());
            users.push(user);
            update_users(users);
        }
        Err(e) => {
            println!("Error hashing password: {}", e);
        }
    }
}

fn login(){
    let email = read_input("Email: ");
    std::io::stdout().flush().expect("Error cleaning buffer.");
    let typed_password = read_password().expect("Failed to read password.");
    let typed_password = hash_password(&typed_password).expect("Error hashing password");
    let auth: Auth = Auth { email: email.clone(), password: typed_password };
    match auth.verify_password("password") { // TODO: Search for user in JSON file
        Ok(_) => println!("Password Valid!"),
        Err(e) => println!("Password does not match: {}", e)
    }
}

fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

#[allow(dead_code)]
fn verify_password(stored_hash: &str, password: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, stored_hash)
}

#[allow(dead_code)]
fn prompt_password() -> Result<String, String>{
    const MAX_ATTEMPTS: u8 = 3;
    let mut attempts: u8 = 0;
    let result = loop {
        print!("Password: ");
        std::io::stdout().flush().expect("Error cleaning buffer.");

        if attempts == MAX_ATTEMPTS {
            break Err("Try too many times. Try again later.".to_string());
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

fn update_users(users: Vec<HashMap<String, &str>>) -> () {
    match serde_json::to_string_pretty(&users) {
        Ok(json_str) => {
            if let Err(e) = fs::write("users.json", json_str) {
                eprintln!("Erro ao salvar arquivo: {}", e);
            }
        },
        Err(e) => eprintln!("Erro ao serializar JSON: {}", e),
    }
}

/* fn validate_email(email: &str) -> Result<(), String> {
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
} */