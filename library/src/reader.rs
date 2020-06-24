use tokio::net::tcp::{OwnedReadHalf};
use crate::protocol::{ReadProtocol};
use std::sync::{Arc};
use futures::lock::{Mutex};
use crate::command::Command;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::RecvError;
use tokio::sync::broadcast::Sender;
use std::collections::HashMap;
use avro_rs::types::Value;
use tokio::io::AsyncReadExt;

pub struct Reader {
    pending_acknowledgements: Mutex<HashMap<Vec<u8>, oneshot::Sender<bool>>>,
    subscriptions: HashMap<Vec<u8>, Sender<Value>>
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            pending_acknowledgements: Mutex::new(HashMap::new()),
            subscriptions: HashMap::new()
        }
    }
    pub async fn read(reader: Arc<Reader>, stream_reader: Arc<Mutex<OwnedReadHalf>>) {
        let mut stream_reader = stream_reader.lock().await;

        loop {
            match stream_reader.read_command().await {
                Ok(Command::Ack) => {
                    println!("Command: Ack");
                    let id: Vec<u8> = stream_reader.read_ack().await;

                    println!("Ack for {:?}", &id);
                    
                    let mut pending_acknowledgements = reader.pending_acknowledgements.lock().await;
                    let sender = pending_acknowledgements.remove(&id).unwrap();
                    sender.send(true).unwrap();

                    ();
                },
                Ok(Command::Nack) => {
                    println!("Command: Nack");
                    let id: Vec<u8> = stream_reader.read_ack().await;

                    println!("Nack for {:?}", &id);
                    
                    let mut pending_acknowledgements = reader.pending_acknowledgements.lock().await;
                    let sender = pending_acknowledgements.remove(&id).unwrap();
                    sender.send(false).unwrap();

                    ();
                },
                Ok(Command::AddModel) => {
                    println!("Command: AddModel");

                    /* Get ID: */
                    let id: Vec<u8> = stream_reader.read_ack().await;

                    ();
                },
                Ok(Command::Push) => {
                    println!("Command: Push");

                    /* Get message ID: */
                    let mut id  = [0u8; 8];
                    stream_reader.read_exact(&mut id).await;

                    /* Get message type: */
                    let mut id  = [0u8; 4];
                    stream_reader.read_exact(&mut id).await;

                    /* Get subscription id: */
                    let mut id  = Vec::<u8>::with_capacity(4);
                    stream_reader.read_exact(&mut id).await;

                    /* Get message: */
                    let (message, length): (Vec<u8>, u32) = stream_reader.read_message().await;

                    
                    match reader.subscriptions.get(&id) {
                        Some(subscriber) => {
                            //subscriber.send(message);
                        },
                        None => {}
                    }
                    
                    ();
                },
                _ => {
                    break;
                }
            }
        
        }
    }
    pub async fn expect_ack(&self, id: Vec<u8>) -> Result<bool, RecvError> {
        let (sender, receiver): (oneshot::Sender<bool>, oneshot::Receiver<bool>) = oneshot::channel();
        {
                let mut pending_acknowledgements = self.pending_acknowledgements.lock().await;
                pending_acknowledgements.insert(id.clone(), sender);
        }
        
        receiver.await
    }
}