use std::io::{self, Write};

fn main(){
    read_input();
    let tastes: [&str; 5] = ["sweet", "little sweet", "spicy", "sour", "salty"];
    let fruits: [&str; 5] = ["apple", "banana", "cherry", "orange", "pineapple"];
    evaluate_fruit(fruits, tastes);
    start_loop(fruits);
}

fn start_loop(fruits: [&str; 5]) {
    let mut index = 0;

    loop {
        if index > 0 {
            println!("Alright! Anything else? (quit to finish)");
        } else {
            println!("Choose your favorite fruit ({})", fruits.join(", "));
        }

        let fruit = read_fruit_line();
        if fruit == "quit" {
            break;
        }

        let found = check_for_fruit(&fruit, fruits);
        if !found {
            println!("{} is not in the list", fruit);
            break;
        }

        index += 1;
    }
}

fn check_for_fruit(fruit: &str, fruits: [&str; 5]) -> bool {
    for i in 0..fruits.len() {
        if fruits[i] == fruit {
            return true;
        }
    }
    false
}

fn read_fruit_line() -> String {
    let mut fruit = String::new();
    io::stdin()
        .read_line(&mut fruit)
        .expect("Failed to read line");

    let fruit = fruit.trim();
    return fruit.to_string();
}

fn evaluate_fruit(fruits: [&str; 5], taste: [&str; 5]){
    for (i, _) in fruits.iter().enumerate(){
        println!("{} is {}", fruits[i], taste[i]);
    }
}

fn read_input() -> String {
    print!("Say my name: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let input = name.trim();
    return input.to_string();
}

