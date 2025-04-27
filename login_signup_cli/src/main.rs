use core::panic;
use std::fs::File;
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

fn validate_password(password: &str){
    // hash implementation
}

// ====================== ALGORITHMS / LEETCODE =======================================
#[allow(dead_code)]
fn search_for_value(arr: &mut Vec<usize>, value: usize) -> bool {
    arr.sort();
    if arr.is_empty() {
        return false;
    }

    let mut start = 0;
    let mut end = arr.len() - 1;

    while start <= end {
        let mid = (start + end) / 2;

        if value == arr[mid] {
            return true;
        }
        else if value > arr[mid] {
            start = mid + 1;
        }
        else if value < arr[mid] {
            if mid == 0 {
                break;
            }

            end = mid - 1;
        }
    }
    return false;
}

#[allow(dead_code)]
fn remove_value_from_array(arr: &mut Vec<usize>, value: usize){
    // My Answer
    let mut right = arr.iter().len() - 1;
    let mut left = 0;

    while right >= left {
        if arr[left] == value {
            for i in (0..left).rev() {
                arr.swap(i, i + 1);
            }
            arr[0] = 0;
        }

        if arr[right] == value {
            for i in right..arr.iter().len() - 1 {
                arr.swap(i, i + 1);
            }

            if let Some(last) = arr.last_mut(){
                *last = 0;
            }
        }

        right -= 1;
        left += 1;
    }
}

#[allow(dead_code)]
fn remove_value_from_array2(arr: &mut Vec<usize>, value: usize) {
    // Best Answer
    let mut i = 0;

    for j in 0..arr.len() {
        if arr[j] != value {
            arr[i] = arr[j];
            i += 1;
        }
    }

    arr.truncate(i);
}

#[allow(dead_code)]
fn sum_numbers_two_pointer() -> usize {
    let end: i32 = 100;
    let mut mid_right: i32 = end / 2;
    let mut mid_left: i32 = end / 2 - 1;
    let mut file = File::create("data.txt").expect("Create failed");

    while mid_right - mid_left <= end {
        let result: i32 = sum_values(mid_right, mid_left);
        let mid_left_odd: bool = is_odd(mid_left);
        let mid_right_odd: bool = is_odd(mid_right);

        if mid_left_odd {
            mid_left -= 1;
        }

        if mid_right_odd {
            mid_right += 2;
        }

        print!("{} + {} = {} \n", mid_left, mid_right, result);
        file.write_all(format!("{} + {} = {}\n", mid_left, mid_right, result).as_bytes()).expect("Write failed");
    }
    return 0
}

#[allow(dead_code)]
fn sum_values(a: i32, b: i32) -> i32 {
    return a + b;
}

#[allow(dead_code)]
fn is_odd(n: i32) -> bool {
    if n % 2 == 0 {
        return true
    }
    return false
}
