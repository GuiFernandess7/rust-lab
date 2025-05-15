use serde::{Serialize, Deserialize};
use reqwest;
use dotenv::dotenv;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct BusAPI {
    line: String,
    lat: f32,
    lon: f32
}

impl BusAPI {
    fn new(line: String, lat: f32, lon: f32, debug: bool) -> Self {
        let instance = BusAPI {line, lat, lon};

        if debug {
            println!("API set successfully! Coordinates: {}, {}", 
                instance.lat, instance.lon);
        }
        return instance
    }

    fn authenticate(&self, token: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post("http://api.olhovivo.sptrans.com.br/v2/Login/Autenticar")
            .query(&[("token", token)])  
            .header("Content-Type", "application/json")
            .body("{}") 
            .send()?;

        if response.status().is_success() {
            let authenticated: bool = response.json()?;
            println!("Authentication status: {}", authenticated);
            Ok(authenticated)
        } else {
            let error_msg = format!("Authentication failed. Status: {}", response.status());
            println!("Error: {}", error_msg);
            Err(error_msg.into())
        }
    }
}

pub fn call_api(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    
    if response.status().is_success() {
        let body = response.text()?;
        //println!("Response: {}", body);
        Ok(body)
    } else {
        let error_msg = format!("Error calling API, status: {}", response.status());
        println!("Error: {}", error_msg);
        Err(error_msg.into())
    }
}

fn main() {
    dotenv().ok();
    let api_instance = BusAPI::new(
        String::from("Line 1"),
        -23.550520,
        -46.633308,
        true
    );
    let api_token = env::var("API_KEY").expect("API_KEY não está definida");
    match api_instance.authenticate(&api_token) {
        Ok(true) => println!("Successfully authenticated!"),
        Ok(false) => println!("Authentication failed - invalid credentials"),
        Err(e) => println!("Error during authentication: {}", e),
    }
}
