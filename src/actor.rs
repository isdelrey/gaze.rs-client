use crate::gaze::Gaze;
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    name: String,
    age: i64
}

pub async fn run() {
    /* Report start: */
    println!("Actor running...");

    /* Connect: */
    let mut gaze = Gaze::connect().await.unwrap();


    /* Add model: */
    let raw_model = r#"
        {
            "type": "record",
            "namespace": "my.company",
            "name": "user_created",
            "fields": [
                {"name": "name", "type": "string"},
                {"name": "age", "type": "long"}
            ]
        }
    "#;
    let id = gaze.add_model(raw_model).await.unwrap();
    println!("Added model with id {}", id);


    /* Publish: */
    for _ in 1..100000 {
        let message = Message {
            name: "Ivo".to_string(),
            age: 24
        };
        match gaze.publish("my.company.user_created".to_string(), message).await {
            Ok(()) => {
                println!("Publish finished by receiving ACK");
            },
            _ => {
                println!("Published failure due to no ACK reception")
            }
        }
        
    }


    /* Report start: */
    println!("Actor finished running");

    ()
}