use gaze::{Gaze, message_type, message::WithMessageType};
use serde::Serialize;
use serde_json::json;

#[message_type("my.company.user_created")]
struct UserCreated {
    name: String,
    age: i64
}

pub async fn run() -> Result<(), ()> {
    /* Report start: */
    println!("Subscriber running...");

    /* Connect: */
    let mut gaze = Gaze::connect().await.unwrap();

    
    /* Add model: */
    gaze.add_model(json!({
        "type": "record",
        "namespace": "my.company",
        "name": "user_created",
        "fields": [
            {"name": "name", "type": "string"},
            {"name": "age", "type": "long"}
        ]
    })).await.unwrap();


    /* Subscribe: */
    gaze.subscribe(json!([{
        "$type": "",
        "age": {"$gt": 20},
        "name": "Ivo"
    }]), 0).await?;

    // gaze.subscribe(filter!{
    //     UserCreated if name == "Ivo" && age > 20
    // });


    Ok(())
}