#![feature(async_closure)]

pub mod protocol;
pub mod numbers;
pub mod time;
pub mod command;
pub mod reader;

use std::error::Error;
use std::boxed::Box;
use tokio::net::TcpStream;
use protocol::{ReadProtocol, WriteProtocol};
use avro_rs::{Schema, to_avro_datum, to_value};
use serde_json::{Value};
use tokio::net::tcp::{OwnedWriteHalf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use futures::lock::Mutex;
use serde::ser::Serialize;
use command::Command;
use std::sync::Arc;
use std::collections::HashMap;
use reader::Reader;
use std::time::SystemTime;
use crate::gaze::time::SmallestReadableString;
use rand::{Rng, thread_rng};
use std::iter;
use rand::distributions::Alphanumeric;

pub struct Gaze {
    pub writer: Arc<Mutex<OwnedWriteHalf>>,
    pub reader: Arc<Reader>,
    models: HashMap<String, Schema>
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
        tokio::spawn(Reader::read(reader.clone(),  stream_reader.clone()));

        Ok(Gaze {
            writer,
            reader,
            models: HashMap::new()
        })
    }
    fn generate_id() -> Vec<u8> {
        let mut ns = [0u8; 6];
        let ns = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().to_smallest_readable_string(&mut ns);

        let random: String;
        {

            let mut rng = thread_rng();
            random = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric) )
            .take(4).collect();
        }

        let id = [ns, random.as_bytes()].concat();
        println!("Generated id: {:?}", id);

        id
    }
    pub async fn publish<T: Serialize>(&self, model_id: String, message: T) -> Result<(), Box<dyn Error>> {
        let id = Gaze::generate_id();
        {
            let mut writer = self.writer.lock().await;

            let model: &Schema = self.models.get(&model_id).unwrap();
            

            /* Validate and write model: */
            let encoded_message = to_avro_datum(model, to_value(message).unwrap()).unwrap();
            
            /* Get message id: */
            writer.write_command(Command::Publish).await;

            /* Get message id: */
            let id = writer.write_id(&id).await;

            /* Write length: */
            writer.write_size(encoded_message.len()).await;

            /* Write message: */
            println!("{:?} {}", encoded_message.as_slice(), std::str::from_utf8(encoded_message.as_slice()).unwrap());
            writer.write(encoded_message.as_slice()).await.unwrap();
        }

        self.reader.expect_ack(id).await.unwrap();

        Ok(())
    }
    pub async fn add_model(&mut self, raw_definition: &str) -> Result<String, Box<dyn Error>> {
        let definition: Value = serde_json::from_str(raw_definition)?;

        let root_name = match definition.get("name") {
            Some(value) if value.is_string() => value.as_str().unwrap().to_string(),
            _ => "".to_string()
        };
         let root_namespace = match definition.get("namespace") {
            Some(value) if value.is_string() => [value.as_str().unwrap(), "."].concat(),
            _ => "".to_string()
        };
        let model: Schema = Schema::parse(&definition)?;

        let id = [root_namespace, root_name].concat();
        self.models.insert(id.clone(), model.clone());
        println!("{}: {:?}", id, model);
        
        /*{
            let mut writer = self.writer.lock().await;

            /* Get message id: */
            writer.write_command(Command::AddModel).await;

            /* Get message id: */
            let id = writer.write_id(&id).await;

            /* Write length: */
            writer.write_size(raw_definition.len()).await;

            /* Write message: */
            println!("{:?} {}", raw_definition, std::str::from_utf8(encoded_message.as_slice()).unwrap());
            writer.write(raw_definition).await.unwrap();
        }*/

        Ok(id)
    }
}