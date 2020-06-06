mod gaze;
mod actor;

use std::thread;
use futures::future::{join_all};

#[tokio::main]
async fn main() {
    let mut handles = Vec::with_capacity(100);
    /* Run actors: */
    for _ in 1..1000 {
        handles.push(tokio::spawn(actor::run()));
    }

    join_all(handles).await;

    thread::park();

    ()
}