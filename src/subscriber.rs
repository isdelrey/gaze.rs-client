use gaze::Gaze;
use serde::Serialize;
use serde_json::json;


#[derive(Serialize)]
struct Message {
    name: String,
    age: i64
}

pub async fn run() {
    /* Report start: */
    println!("Subscriber running...");

    /* Connect: */
    let mut gaze = Gaze::connect().await.unwrap();

    
    /* Add model: */
    let id = gaze.add_model(json!({
        "type": "record",
        "namespace": "my.company",
        "name": "user_created",
        "fields": [
            {"name": "name", "type": "string"},
            {"name": "age", "type": "long"}
        ]
    })).await.unwrap();


    /* Subscribe: */
    gaze.subscribe(json!({
        "type": "record",
        "namespace": "my.company",
        "name": "user_created",
        "fields": [
            {"name": "name", "type": "string"},
            {"name": "age", "type": "long"}
        ]
    })).await;

    ()
}