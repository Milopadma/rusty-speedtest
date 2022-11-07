// all in one CLI tool to check wifi connections (speed, latency, etc.)
// crate use imports
use std::time::Instant;
use reqwest::Client;

// function to check for the speed of the connection by downloading a file 
async fn checkSpeed: f64 () {

    // establish a connection to the server
    let client = Client::new();
    let mut response = client.get("https://cachefly.cachefly.net/100mb.test").send().await.unwrap();

    // check the status code
    // if the status code is not 200, then the connection failed
    if response.status() != 200 {
        println!("Connection failed");
    }

    // if the status code is 200, then the connection was successful
    else {
        println!("Connection successful");
    }

    // get the time it took to download the failed
    let start = Instant::now();
    let mut buf: Vec<T> = Vec::new();

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}


fn main(){
    
    
}
