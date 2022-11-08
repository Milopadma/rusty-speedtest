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
async fn check_speed<T>() {

    // get the time it took to download the file
    let start = Instant::now();
    // establish a connection to the server
    let client = Client::new();
    let  response = client.get("https://cachefly.cachefly.net/1000mb.test").send().await.unwrap();
    if response.status() != 200 {
        println!("Connection failed");
    }

    // if the status code is 200, then the connection was successful
    else {
        println!("Connection successful");
    }
    let _ = response.bytes().await.unwrap();
    let duration = start.elapsed();
    
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

//macro to make this main function async
#[tokio::main]
async fn main(){
    // call the function
    check_speed::<T>().await; 
    
}
