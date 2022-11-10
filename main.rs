// all in one CLI tool to check wifi connections (speed, latency, etc.)
// crate use imports
use std::time::Instant;
use reqwest::Client;
use tokio;

// show how to implement a Type
struct T {
    name: String,
    speed: u32,
    latency: u32,
}

// function to check for the speed of the connection by downloading a file
async fn check_speed() {
    // get the time it took to download the file
    let start = Instant::now();
    // establish a connection to the server
    let client = Client::new();
    let response = client.get("https://cachefly.cachefly.net/10mb.test").send().await.unwrap();
    if response.status() != 200 {
        println!("Connection failed");
    } else {
        // if the status code is 200, then the connection was successful
        println!("Connection successful");
    }
    let _ = response.bytes().await.unwrap();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

//function to check for default gateways
async fn check_gateway() {
    // ping each known default ip addresses and check if it is reachable
    let client = Client::new();
    let response = client.get("192.168.20.1").send().await;
    match response {
        Ok(_) => println!("Gateway is reachable"),
        Err(_) => println!("Gateway is not reachable"),
    }
}
    

// for the CLI selection menu
enum Menu {
    Speed,
    Gateway,
    Exit,
}

//macro to make this main function async
#[tokio::main]
async fn main() {

    // menu selection
    println!("Select an option:");
    println!("1. Check speed");
    println!("2. Check gateway");
    println!("3. Exit");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: u32 = input.trim().parse().unwrap();

    // match the input to the enum
    match input {
        1 => check_speed().await,
        2 => check_gateway().await,
        3 => println!("Exiting..."),
        _ => println!("Invalid input"),
    }

}
