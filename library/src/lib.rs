pub mod command;
pub mod message;
pub mod numbers;
pub mod protocol;
pub mod reader;

use avro_rs::{to_avro_datum, to_value, types::Value, Schema};
use command::Command;
use fasthash::xx;
use futures::lock::Mutex;
pub use gaze_macros::message_type;
use message::WithMessageType;
use protocol::{ReadProtocol, WriteProtocol};
use rand::thread_rng;
use rand::RngCore;
use reader::Reader;
use serde::ser::Serialize;
use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tokio::sync::RwLock;

pub struct Gaze {
    pub writer: Arc<Mutex<OwnedWriteHalf>>,
    pub reader: Arc<Reader>,
    schemas: Arc<RwLock<HashMap<Vec<u8>, Schema>>>,
}

impl Gaze {
    pub async fn connect() -> Result<Gaze, Box<dyn Error>> {
        let host = std::env::var("HOST").unwrap();
        //println!("About to connect to Gaze on {}", host);

        let stream: TcpStream = match TcpStream::connect(host.clone()).await {
            Ok(stream) => {
                //println!("Connected to Gaze on {}", host);
                stream
            }
            Err(error) => {
                //println!("Cannot connect to server: {}", error);
                return Err(Box::new(error));
            }
        };

        /* Split stream: */
        let (stream_reader, writer) = stream.into_split();
        let stream_reader = Arc::new(Mutex::new(stream_reader));
        let writer = Arc::new(Mutex::new(writer));

        /* Create reader: */
        let reader = Arc::new(Reader::new());

        /* Create schemas: */
        let schemas = Arc::new(RwLock::new(HashMap::new()));
        
        /* Spawn reader: */
        tokio::spawn(Reader::read(reader.clone(), stream_reader.clone(), schemas.clone()));

        Ok(Gaze {
            writer,
            reader,
            schemas,
        })
    }
    fn generate_message_id() -> Vec<u8> {
        let timestamp_as_u64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
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
    fn generate_subscription_id() -> Vec<u8> {
        let timestamp_as_u64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let mut timestamp = timestamp_as_u64.to_le_bytes();

        {
            let mut rng = thread_rng();
            rng.fill_bytes(&mut timestamp[2..4]);
        }

        let id = &timestamp[2..6];
        Vec::from(id)
    }
    pub fn hash_message_type(message_type: String) -> Vec<u8> {
        Vec::from(&xx::hash32(message_type.as_bytes()).to_le_bytes()[..])
    }
    pub async fn subscribe(
        &self,
        filter: serde_json::Value,
        offset: u64,
    ) -> Result<Receiver<Value>, ()> {
        let (sender, receiver) = channel::<avro_rs::types::Value>(1000);

       {
            let mut writer = self.writer.lock().await;
            
        
        /* Write command: */
        writer.write_command(Command::Subscribe).await;

        /* Write offset: */
        let raw_offset = &offset.to_le_bytes()[2..8];
        writer.write_id(&raw_offset).await;
        //println!("Offset: {} -> {:?}", offset, raw_offset);

        /* Write subscription id: */
        let id = Gaze::generate_subscription_id();

        //println!("Subscription {:?} with filter {:?} with offset {} requested", id, filter, offset);

        writer.write_id(&id).await;

        let filter = match filter {
            serde_json::Value::Array(_) => filter,
            serde_json::Value::Object(_) => serde_json::Value::Array(vec![filter]),
            _ => return Err(()),
        };

        let raw_schema: String = filter.to_string();

        //println!("Filter size: {} -> {:?}", raw_schema.len(), (raw_schema.len() as u32).to_le_bytes());

        /* Write raw schema size: */
        writer
            .write(&(raw_schema.len() as u32).to_le_bytes())
            .await
            .unwrap();

        /* Write raw schema: */
        writer.write(raw_schema.as_bytes()).await.unwrap();

        /* Register subscription: */
        let mut subscriptions = self.reader.subscriptions.write().await;
        subscriptions.insert(id.clone(), sender);
       }


        //println!("Subscription {:?} request completed", id);

        Ok(receiver)
    }
    pub async fn publish(
        &self,
        encoded_message: Vec<u8>,
        message_type: Vec<u8>
    ) -> Result<(), Box<dyn Error>> {
        let id = Gaze::generate_message_id();
        let schemas = self.schemas.read().await;
        let schema: &Schema = schemas.get(&message_type).unwrap();


        {

            let mut writer = self.writer.lock().await;

            /* Write command: */
            writer.write_command(Command::Message).await;
            //println!("Sending message command: {:?}", Command::Message);

            /* Write message id: */
            writer.write_id(&id).await;
            //println!("Message id: {:?}", id);

            /* Write message type: */
            writer.write(&message_type).await.unwrap();
            //println!("Message type: {:?}", message_type);

            /* Write length: */
            writer.write_size(encoded_message.len()).await;

            /* Write message: */
            writer.write(&encoded_message).await.unwrap();
        }

        //self.reader.expect_ack(&id).await.unwrap();
        //println!("Delivery confirmed for {:?}", &id);

        Ok(())
    }
    pub async fn add_type(
        &mut self,
        raw_definition: &str
    ) -> Result<Vec<u8>, Box<dyn Error>> {

        let definition: serde_json::Value = serde_json::from_str(raw_definition)?;

        let root_name = match definition.get("name") {
            Some(value) if value.is_string() => value.as_str().unwrap().to_string(),
            _ => "".to_string(),
        };
        let root_namespace = match definition.get("namespace") {
            Some(value) if value.is_string() => [value.as_str().unwrap(), "."].concat(),
            _ => "".to_string(),
        };

        let schema: Schema = Schema::parse(&definition)?;

        let full_message_type = [root_namespace, root_name].concat();

        let message_type = Gaze::hash_message_type(full_message_type.clone());
        //println!("Message type: {} -> {:?}", full_message_type.clone(), message_type.clone());
        
        let mut schemas = self.schemas.write().await;

        if let Some(_) = schemas.get(&message_type) {
            return Ok(message_type)
        }

        schemas.insert(message_type.clone(), schema.clone());
        //println!("{:?}: {:?}", &message_type, schema);

        {
            let mut writer = self.writer.lock().await;

            /* Write command */
            writer.write_command(Command::Schema).await;

            /* Write message type: */
            writer
                .write(&message_type[..])
                .await
                .expect("Cannot write message type");

            /* Write length: */
            writer
                .write(&(raw_definition.len() as u32).to_le_bytes())
                .await
                .expect("Cannot write length");
            
            //println!("Schema size: {} -> {:?}", raw_definition.len(), (raw_definition.len() as u32).to_le_bytes());
            //println!("Schema: {:?}", raw_definition.as_bytes());

            /* Write message: */
            writer
                .write(raw_definition.as_bytes())
                .await
                .expect("Cannot write message");
        }

        Ok(message_type)
    }
}
