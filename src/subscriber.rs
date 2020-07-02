use gaze::{message::WithMessageType, message_type, Gaze};
use tokio::sync::mpsc::{Receiver};
use serde::Serialize;
use serde_json::json;

#[message_type("my.company.user_created")]
struct UserCreated {
    name: String,
    age: i64,
}

pub async fn run() {
    /* Report start: */
    println!("Subscriber running...");

    /* Connect: */
    let mut gaze = Gaze::connect().await.unwrap();

    /* Add model: */
    gaze.add_type(
        r#"{
        "type": "record",
        "namespace": "my.company",
        "name": "user_created",
        "fields": [
            {"name": "name", "type": "string"},
            {"name": "age", "type": "int"}
        ]
    }"#
    )
    .await
    .unwrap();

    /* Subscribe: */
    let mut messages: Receiver<avro_rs::types::Value> = gaze.subscribe(
        json!([{
            "$type": "my.company.user_created",
            "age": {"$lt": 25},
            "name": {"$ew": "o"}
        }]),
        0,
    )
    .await.unwrap();

    while let Some(message) = messages.recv().await {
        println!("-> {:?} received", message);
    }

    println!("Subscriber finished")

    // gaze.subscribe(filter!{
    //     UserCreated if name == "Ivo" && age > 20
    // });

}
