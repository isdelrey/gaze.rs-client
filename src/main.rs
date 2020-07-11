mod producer;
mod subscriber;

use std::thread;
use futures::future::{join_all};
use tokio::time::delay_for;

#[tokio::main]
async fn main() {
    tokio::spawn(subscriber::run());
    tokio::spawn(producer::run());
    // loop {
    //     for _ in 1..200 { tokio::spawn(producer::run()); }
    //     delay_for(std::time::Duration::from_secs(30)).await;
    // }

    thread::park();
    
    ()
}