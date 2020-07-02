mod producer;
mod subscriber;

use std::thread;
use futures::future::{join_all};

#[tokio::main]
async fn main() {
    if std::env::var("ROLE").unwrap_or("".to_string()) == "subscriber" {
        tokio::spawn(subscriber::run());
    }
    else {
        let producers: usize = std::env::var("PRODUCERS").unwrap_or("1".to_string()).parse().expect("Producers env var is not a number");
        for i in 1..producers + 1 {
            tokio::spawn(producer::run());
        }
    }

    thread::park();
    ()
}