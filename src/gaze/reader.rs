use tokio::net::tcp::{OwnedReadHalf};
use crate::gaze::protocol::{ReadProtocol};
use std::sync::{Arc};
use futures::lock::{Mutex};
use crate::gaze::command::Command;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::RecvError;
use std::collections::HashMap;

pub struct Reader {
    pending_acknowledgements: Mutex<HashMap<Vec<u8>, oneshot::Sender<bool>>>
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            pending_acknowledgements: Mutex::new(HashMap::new())
        }
    }
    pub async fn read(reader: Arc<Reader>, stream_reader: Arc<Mutex<OwnedReadHalf>>) {
        let mut stream_reader = stream_reader.lock().await;

        loop {
            match stream_reader.read_command().await {
                Ok(Command::Ack) => {
                    println!("Command: Ack");
                    let id: Vec<u8> = stream_reader.read_ack().await;

                    println!("Ack for {}", std::str::from_utf8(&id).unwrap());
                    
                    let mut pending_acknowledgements = reader.pending_acknowledgements.lock().await;
                    let sender = pending_acknowledgements.remove(&id).unwrap();
                    sender.send(true).unwrap();

                    ();
                },
                Ok(Command::Nack) => {
                    println!("Command: Nack");
                    let id: Vec<u8> = stream_reader.read_ack().await;

                    println!("Nack for {}", std::str::from_utf8(&id).unwrap());
                    
                    let mut pending_acknowledgements = reader.pending_acknowledgements.lock().await;
                    let sender = pending_acknowledgements.remove(&id).unwrap();
                    sender.send(false).unwrap();

                    ();
                },
                Ok(Command::AddModel) => {
                    println!("Command: AddModel");
                    let id: Vec<u8> = stream_reader.read_ack().await;

                    println!("AddModel for {}", std::str::from_utf8(&id).unwrap());
                    
                    let mut pending_acknowledgements = reader.pending_acknowledgements.lock().await;
                    let sender = pending_acknowledgements.remove(&id).unwrap();
                    sender.send(false).unwrap();

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