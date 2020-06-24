#![feature(async_closure)]

pub mod protocol;
pub mod numbers;
pub mod command;
pub mod reader;
pub mod message;

use std::error::Error;
use std::boxed::Box;
use tokio::net::TcpStream;
use protocol::{ReadProtocol, WriteProtocol};
use avro_rs::{Schema, to_avro_datum, to_value, types::Value};
use tokio::net::tcp::{OwnedWriteHalf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use futures::lock::Mutex;
use serde::ser::Serialize;
use command::Command;
use std::sync::Arc;
use std::collections::HashMap;
use reader::Reader;
use std::time::SystemTime;
use rand::{thread_rng};
use rand::RngCore;
use fasthash::{xx};
use std::sync::mpsc::{Sender, Receiver, channel};
pub use gaze_macros::message_type;
use message::WithMessageType;

pub struct Gaze {
    pub writer: Arc<Mutex<OwnedWriteHalf>>,
    pub reader: Arc<Reader>,
    models: HashMap<Vec<u8>, Schema>
}

impl Gaze {
    pub async fn connect() -> Result<Gaze, Box<dyn Error>> {
        let host = std::env::var("HOST").unwrap();
        println!("About to connect to Gaze on {}", host);

        let stream: TcpStream = match TcpStream::connect(host.clone()).await {
            Ok(stream) => {
                println!("Connected to Gaze on {}", host);
                stream
            },
            Err(error) => {
                println!("Cannot connect to server: {}", error);
                return Err(Box::new(error))
            }
        };

        /* Split stream: */
        let (stream_reader, writer) = stream.into_split();
        let stream_reader = Arc::new(Mutex::new(stream_reader));
        let writer = Arc::new(Mutex::new(writer));

        /* Create reader: */
        let reader = Arc::new(Reader::new());

        /* Spawn reader: */
        tokio::spawn(Reader::read(reader.clone(), stream_reader.clone()));

        Ok(Gaze {
            writer,
            reader,
            models: HashMap::new()
        })
    }
    fn generate_id() -> Vec<u8> {
        let timestamp_as_u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64;
        let mut timestamp = timestamp_as_u64.to_le_bytes();

        {
            let mut rng = thread_rng();
            rng.fill_bytes(&mut timestamp[1..4]);
        }

        let last_byte_random = timestamp[1];
        let last_byte_mask = 0b1111_1100;
        timestamp[2] = last_byte_random | last_byte_mask;

        let id = &timestamp[2..8];
        
        Vec::from(id)
    }
    fn hash_message_type(message_type: String) -> Vec<u8> {
        Vec::from(&xx::hash32(message_type.as_bytes()).to_le_bytes()[..])
    }
    pub async fn subscribe(&self, filter: serde_json::Value) -> Receiver<Value> {
        let (sender, receiver) = channel::<Value>();

        receiver
    }
    pub async fn publish<T: Serialize + WithMessageType>(&self, message: T) -> Result<(), Box<dyn Error>> {
        let id = Gaze::generate_id();

        {
            let mut writer = self.writer.lock().await;

            let message_type = Gaze::hash_message_type(message.get_message_type());

            let model: &Schema = self.models.get(&message_type).unwrap();
            

            /* Validate and write model: */
            let encoded_message = to_avro_datum(model, to_value(message).unwrap()).unwrap();
            
            /* Write command: */
            writer.write_command(Command::Publish).await;

            /* Write message id: */
            let id = writer.write_id(&id).await;

            /* Write message id: */
            writer.write(&message_type).await.unwrap();

            /* Write length: */
            writer.write_size(encoded_message.len()).await;

            /* Write message: */
            println!("{:?} {}", encoded_message.as_slice(), std::str::from_utf8(encoded_message.as_slice()).unwrap());
            writer.write(encoded_message.as_slice()).await.unwrap();
        }

        //self.reader.expect_ack(id).await.unwrap();

        Ok(())
    }
    pub async fn add_model(&mut self, definition: serde_json::Value) -> Result<Vec<u8>, Box<dyn Error>> {

        let root_name = match definition.get("name") {
            Some(value) if value.is_string() => value.as_str().unwrap().to_string(),
            _ => "".to_string()
        };
         let root_namespace = match definition.get("namespace") {
            Some(value) if value.is_string() => [value.as_str().unwrap(), "."].concat(),
            _ => "".to_string()
        };
        let model: Schema = Schema::parse(&definition)?;

        let full_message_type = [root_namespace, root_name].concat();
        let message_type = Gaze::hash_message_type(full_message_type);
        self.models.insert(message_type.clone(), model.clone());
        println!("{:?}: {:?}", &message_type, model);
        
        /*{
            let mut writer = self.writer.lock().await;

            /* Get message id: */
            writer.write_command(Command::AddModel).await;

            /* Get message id: */
            let id = writer.write_id(&message_type[..]).await;

            /* Write length: */
            writer.write_size(raw_definition.len()).await;

            /* Write message: */
            println!("{:?} {}", raw_definition, std::str::from_utf8(encoded_message.as_slice()).unwrap());
            writer.write(raw_definition).await.unwrap();
        }*/

        Ok(message_type)
    }
}