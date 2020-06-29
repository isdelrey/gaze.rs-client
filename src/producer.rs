use gaze::{message::WithMessageType, message_type, Gaze};
use serde::Serialize;
use serde_json::json;

#[message_type("my.company.user_created")]
struct UserCreated {
    name: String,
    age: i32,
}

pub async fn run() {
    /* Report start: */
    println!("Producer running...");

    /* Connect: */
    let mut gaze = Gaze::connect().await.unwrap();
    
    
    /* Add model: */
    gaze.add_type(r#"{
        "type": "record",
        "namespace": "my.company",
        "name": "user_created",
        "fields": [
            {"name": "name", "type": "string"},
            {"name": "age", "type": "int"}
        ]
    }"#).await.unwrap();

    /* Publish: */
    loop {
        let user_created = UserCreated {
            name: "Ivo".to_string(),
            age: 24,
        };

        println!("{:?}", user_created);
        match gaze.publish(user_created).await {
            Ok(()) => {
                println!("Publish finished by receiving ACK");
            }
            _ => println!("Published failure due to no ACK reception"),
        };
        break;
    }
}
