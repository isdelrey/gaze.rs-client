use crate::command::Command;
use crate::protocol::ReadProtocol;
use avro_rs::types::Value;
use futures::lock::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::tcp::OwnedReadHalf;
use tokio::sync::broadcast::Sender;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::RecvError;

pub struct Reader {
    pending_acknowledgements: Mutex<HashMap<Vec<u8>, oneshot::Sender<bool>>>,
    subscriptions: HashMap<Vec<u8>, Sender<Value>>,
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            pending_acknowledgements: Mutex::new(HashMap::new()),
            subscriptions: HashMap::new(),
        }
    }
    pub async fn read(reader: Arc<Reader>, stream_reader: Arc<Mutex<OwnedReadHalf>>) {
        let mut stream_reader = stream_reader.lock().await;

        loop {
            match stream_reader.read_command().await {
                Ok(Command::MessageAck) => {
                    println!("Command: Ack");
                    let id: Vec<u8> = stream_reader.read_ack().await;

                    println!("Ack for {:?}", &id);
                    let mut pending_acknowledgements = reader.pending_acknowledgements.lock().await;
                    let sender = pending_acknowledgements.remove(&id).unwrap();
                    sender.send(true).unwrap();

                    ();
                }
                Ok(Command::MessageNack) => {
                    println!("Command: Nack");
                    let id: Vec<u8> = stream_reader.read_ack().await;

                    println!("Nack for {:?}", &id);
                    let mut pending_acknowledgements = reader.pending_acknowledgements.lock().await;
                    let sender = pending_acknowledgements.remove(&id).unwrap();
                    sender.send(false).unwrap();

                    ();
                }
                Ok(Command::Schema) => {
                    println!("Command: Schema");

                    /* Get ID: */
                    let id: Vec<u8> = stream_reader.read_ack().await;

                    ();
                }
                Ok(Command::SubscriptionMessage) => {
                    println!("Command: SubscriptionMessage");

                    /* Get message ID: */
                    let mut id = [0u8; 8];
                    stream_reader
                        .read_exact(&mut id)
                        .await
                        .expect("Cannot read message id");

                    /* Get message type: */
                    let mut id = [0u8; 4];
                    stream_reader
                        .read_exact(&mut id)
                        .await
                        .expect("Cannot read message type");

                    /* Get subscription id: */
                    let mut id = Vec::<u8>::with_capacity(4);
                    stream_reader
                        .read_exact(&mut id)
                        .await
                        .expect("Cannot read subscription id");

                    /* Get message: */
                    let (message, length): (Vec<u8>, u32) = stream_reader.read_message().await;

                    match reader.subscriptions.get(&id) {
                        Some(subscriber) => {
                            //subscriber.send(message);
                        }
                        None => {}
                    }
                    ();
                }
                _ => {
                    break;
                }
            }
        }
    }
    pub async fn expect_ack(&self, id: Vec<u8>) -> Result<bool, RecvError> {
        let (sender, receiver): (oneshot::Sender<bool>, oneshot::Receiver<bool>) =
            oneshot::channel();
        {
            let mut pending_acknowledgements = self.pending_acknowledgements.lock().await;
            pending_acknowledgements.insert(id.clone(), sender);
        }
        receiver.await
    }
}
