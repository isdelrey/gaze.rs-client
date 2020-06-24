mod producer;
mod subscriber;

use std::thread;
use futures::future::{join_all};

#[tokio::main]
async fn main() {
    for i in 1..500 {
        tokio::spawn(producer::run());
    }

    for i in 1..2 {
        tokio::spawn(subscriber::run());
    }

    thread::park();
    ()
}