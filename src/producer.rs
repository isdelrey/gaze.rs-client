use gaze::{message::WithMessageType, message_type, Gaze};
use serde::Serialize;
use serde_json::json;
use avro_rs::{to_avro_datum, to_value, types::Value, Schema};

#[message_type("my.company.user_created")]
#[derive(Clone)]
struct UserCreated {
    name: String,
    surname: String,
    lemma: String,
    area: String,
    age: i32,
    height: i32,
    weight: i32,
}

pub async fn run() {
    /* Report start: */
    //println!("Producer running...");

    /* Connect: */
    let mut gaze = Gaze::connect().await.unwrap();
    
    
    /* Add model: */
    gaze.add_type(r#"{
        "type": "record",
        "namespace": "my.company",
        "name": "user_created",
        "fields": [
            {"name": "name", "type": "string"},
            {"name": "surname", "type": "string"},
            {"name": "lemma", "type": "string"},
            {"name": "area", "type": "string"},
            {"name": "age", "type": "int"},
            {"name": "height", "type": "int"},
            {"name": "weight", "type": "int"}
        ]
    }"#).await.unwrap();

    /* Publish: */

        let user_created = UserCreated {
            name: "Ivo".to_string(),
            surname: "Sequeros".to_string(),
            lemma: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi quis.Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla risus orci, dapibus sed mollis pharetra, mollis a enim. Mauris diam risus, viverra vel eros sit amet, lobortis pulvinar mi. Aenean vel tincidunt eros. Interdum et malesuada fames ac ante ipsum primis in faucibus. Pellentesque nec consequat leo. Sed ac urna convallis nisi ornare pharetra vitae non sapien. Fusce facilisis metus a lectus.Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla ut lacus ex. Donec luctus justo a porta dignissim. Sed mauris justo, ullamcorper id risus eget, sodales tempus tortor. Quisque et turpis nec augue porttitor bibendum id non urna. Donec tristique purus a nisl vehicula, in vehicula quam sodales. Ut sit amet nulla porttitor, porttitor augue ornare, convallis ex. In molestie dignissim tortor sit amet tempor. Phasellus at porta mauris. Maecenas placerat, nisi ut sodales pulvinar volutpat.Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla ut lacus ex. Donec luctus justo a porta dignissim. Sed mauris justo, ullamcorper id risus eget, sodales tempus tortor. Quisque et turpis nec augue porttitor bibendum id non urna. Donec tristique purus a nisl vehicula, in vehicula quam sodales. Ut sit amet nulla porttitor, porttitor augue ornare, convallis ex. In molestie dignissim tortor sit amet tempor. Phasellus at porta mauris. Maecenas placerat, nisi ut sodales pulvinar volutpat.Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla ut lacus ex. Donec luctus justo a porta dignissim. Sed mauris justo, ullamcorper id risus eget, sodales tempus tortor. Quisque et turpis nec augue porttitor bibendum id non urna. Donec tristique purus a nisl vehicula, in vehicula quam sodales. Ut sit amet nulla porttitor, porttitor augue ornare, convallis ex. In molestie dignissim tortor sit amet tempor. Phasellus at porta mauris. Maecenas placerat, nisi ut sodales pulvinar volutpat.".to_string(),
            area: "EU".to_string(),
            age: 24,
            height: 183,
            weight: 64,
        };



        let definition: serde_json::Value = serde_json::from_str(r#"{
        "type": "record",
        "namespace": "my.company",
        "name": "user_created",
        "fields": [
            {"name": "name", "type": "string"},
            {"name": "surname", "type": "string"},
            {"name": "lemma", "type": "string"},
            {"name": "area", "type": "string"},
            {"name": "age", "type": "int"},
            {"name": "height", "type": "int"},
            {"name": "weight", "type": "int"}
        ]
    }"#).unwrap();

        let schema: Schema = Schema::parse(&definition).unwrap();
    /* Validate and write schema: */
    let encoded_message = to_avro_datum(&schema, to_value(user_created.clone()).unwrap()).unwrap();

    let message_type = Gaze::hash_message_type("my.company.user_created".to_string());
    
    loop {

        //println!("{:?}", user_created);


        gaze.publish(encoded_message.clone(), message_type.clone()).await;
    }
}
