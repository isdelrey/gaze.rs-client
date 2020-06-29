mod producer;
mod subscriber;

use std::thread;
use futures::future::{join_all};

#[tokio::main]
async fn main() {
    for i in 1..2 {
        tokio::spawn(subscriber::run());
    }
    
    for i in 1..2 {
        std::thread::sleep(std::time::Duration::from_secs(5));
        tokio::spawn(producer::run());
    }

    thread::park();
    ()
}