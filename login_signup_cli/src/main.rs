use std::collections::HashMap;
use std::io::{self, Write};
use bcrypt::{hash, DEFAULT_COST, verify};
use rpassword::read_password;
use std::env;

pub trait ValidatorTrait {
    fn validate_email(&self, email: &str) -> Result<(), String>;
    fn validate_password(&self, password: &str) -> Result<(), String>;
    fn hash_password(&self, password: &str) -> Result<String, bcrypt::BcryptError>;
    fn verify_password(&self, password: &str, stored_hash: &str) -> Result<bool, bcrypt::BcryptError>;
}

pub trait CliTrait {
    fn read_generic_input(&self, prompt: &str) -> String;
    fn read_password_input(&self, prompt: &str) -> Result<String, std::io::Error>;
    fn read_email_input(&self) -> String;
}

pub struct Validator;
impl ValidatorTrait for Validator {
    fn validate_email(&self, email: &str) -> Result<(), String> {
        let email_regex = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .map_err(|_| "Invalid regex pattern".to_string())?;

        if !email_regex.is_match(email) {
            Err("Invalid email!".to_string())
        } else {
            Ok(())
        }
    }

    fn validate_password(&self, password: &str) -> Result<(), String> {
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_digit = password.chars().any(|c| c.is_digit(10));
        let has_special = password.chars().any(|c| c.is_ascii_punctuation());

        let min_length = 8;
        if password.len() < min_length
            || !has_lowercase
            || !has_uppercase
            || !has_digit
            || !has_special
        {
            return Err("Invalid password. Must have uppercase, lowercase letters, symbols and numbers.".to_string());
        }

        Ok(())
    }

    fn hash_password(&self, password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password, DEFAULT_COST)
    }

    fn verify_password(&self, password: &str, stored_hash: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, stored_hash)
    }
}

pub struct Cli;
impl CliTrait for Cli {
    fn read_generic_input(&self, prompt: &str) -> String {
        let mut input = String::new();

        print!("{}: ", prompt);
        io::stdout().flush().expect("Failed cleaning buffer");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input.trim().to_string()
    }

    fn read_password_input(&self, prompt: &str) -> Result<String, std::io::Error> {
        print!("{}: ", prompt);
        io::stdout().flush()?;
        read_password()
    }

    fn read_email_input(&self) -> String {
        self.read_generic_input("Email")
    }
}

pub struct Auth<V: ValidatorTrait, C: CliTrait> {
    email: String,
    password: String,
    validator: V,
    cli: C,
}

impl<V: ValidatorTrait, C: CliTrait> Auth<V, C> {
    pub fn new(
        email: String,
        password: String,
        validator: V,
        cli: C,
    ) -> Self {
        Auth {
            email,
            password,
            validator,
            cli,
        }
    }

    fn signup(&mut self) -> Result<(), SignUpError> {
        match self.validator.validate_email(&self.email) {
            Ok(_) => {},
            Err(err) => {
                println!("{}", err);
                return Err(SignUpError::InvalidEmail(err));
            }
        }

        match self.validator.validate_password(&self.password) {
            Ok(_) => println!("Hi {}, your account has been created!", self.email),
            Err(err) => {
                println!("{}", err);
                return Err(SignUpError::InvalidPassword(err));
            }
        }

        match self.validator.hash_password(&self.password) {
            Ok(hashed_password) => {
                let mut users: Vec<HashMap<String, String>> = vec![];
                let mut user: HashMap<String, String> = HashMap::new();
                user.insert(self.email.clone(), hashed_password);
                users.push(user);
                //add_user_to_db(users);
                Ok(())
            }
            Err(e) => {
                println!("Error hashing password: {}", e);
                Err(SignUpError::GenericError(format!("Error hashing password: {}", e)))
            }
        }
    }

    fn login(&mut self) -> Result<(), LoginError> {
        const MAX_ATTEMPTS: u8 = 3;
        let mut attempts = 0;

        let stored_hash = match self.validator.hash_password("pwdtest") {
            Ok(hash) => hash,
            Err(e) => return Err(LoginError::GenericError(format!("Failed to hash password: {}", e))),
        };

        while attempts < MAX_ATTEMPTS {
            match self.validator.verify_password(&self.password, stored_hash.as_str()) {
                Ok(is_valid) => {
                    if is_valid {
                        return Ok(());
                    } else {
                        attempts += 1;
                        println!("Invalid password. Try again.");
                        let password = match self.cli.read_password_input("Password") {
                            Ok(password) => password,
                            Err(_) => return Err(LoginError::GenericError("Error reading password".to_string())),
                        };
                        self.password = password;
                    }
                }
                Err(e) => {
                    return Err(LoginError::GenericError(format!("Error verifying password: {}", e)));
                }
            }
        }

        Err(LoginError::InvalidPassword)
    }
}

enum LoginError {
    InvalidPassword,
    GenericError(String),
}

enum SignUpError {
    InvalidEmail(String),
    InvalidPassword(String),
    GenericError(String),
}

fn main() {
    let validator = Validator {};
    let cli = Cli {};

    let email = cli.read_email_input();
    let password = cli.read_password_input("Password").unwrap();

    let mut auth = Auth::new(email, password, validator, cli);
    let args: Vec<String> = env::args().collect();

    if let Some(value) = args.get(1) {
        match value.as_str() {
            "signup" => {
                if let Err(e) = auth.signup() {
                    match e {
                        SignUpError::InvalidEmail(err) => println!("Email error: {}", err),
                        SignUpError::InvalidPassword(err) => println!("Password error: {}", err),
                        SignUpError::GenericError(err) => println!("Error: {}", err),
                    }
                }
                else {
                    println!("Signup successful!");
                }
            }
            "login" => {
                if let Err(e) = auth.login() {
                    match e {
                        LoginError::InvalidPassword => println!("Invalid password"),
                        LoginError::GenericError(err) => println!("Error: {}", err),
                    }
                } else {
                    println!("Login successful!");
                }
            }
            _ => println!("Unknown command. Use 'signup' or 'login'."),
        }
    }
}
