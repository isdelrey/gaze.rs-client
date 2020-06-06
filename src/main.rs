mod gaze;
mod actor;

use std::thread;
use futures::future::{join_all};

#[tokio::main]
async fn main() {
    actor::run().await;
    ()
}