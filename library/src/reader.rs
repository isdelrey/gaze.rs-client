use crate::command::Command;
use crate::protocol::ReadProtocol;
use avro_rs::{types::Value, from_avro_datum, Schema};
use futures::lock::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::tcp::OwnedReadHalf;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::RecvError;
use tokio::sync::RwLock;


pub struct Reader {
    pending_acknowledgements: Mutex<HashMap<Vec<u8>, oneshot::Sender<bool>>>,
    pub subscriptions: RwLock<HashMap<Vec<u8>, Sender<Value>>>,
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            pending_acknowledgements: Mutex::new(HashMap::new()),
            subscriptions: RwLock::new(HashMap::new()),
        }
    }
    pub async fn read(reader: Arc<Reader>, stream_reader: Arc<Mutex<OwnedReadHalf>>, schemas: Arc<RwLock<HashMap<Vec<u8>, Schema>>>) {
        let mut stream_reader = stream_reader.lock().await;

        //println!("Ready to read command");
        loop {
            match stream_reader.read_command().await {
                Ok(Command::MessageAck) => {
                    //println!("Command: Ack");
                    let id: Vec<u8> = stream_reader.read_ack().await;

                    //println!("Ack for {:?}", &id);
                    let mut pending_acknowledgements = reader.pending_acknowledgements.lock().await;
                    let sender = pending_acknowledgements.remove(&id).unwrap();
                    //sender.send(true).unwrap();

                    ();
                },
                Ok(Command::MessageNack) => {
                    //println!("Command: Nack");
                    let id: Vec<u8> = stream_reader.read_ack().await;

                    //println!("Nack for {:?}", &id);
                    let mut pending_acknowledgements = reader.pending_acknowledgements.lock().await;
                    let sender = pending_acknowledgements.remove(&id).unwrap();
                    sender.send(false).unwrap();

                    ();
                },
                Ok(Command::Schema) => {
                    //println!("Command: Schema");

                    /* Get ID: */
                    let id: Vec<u8> = stream_reader.read_ack().await;

                    ();
                },
                Ok(Command::SubscriptionMessage) => {
                    //println!("Command: SubscriptionMessage");

                    /* Get message ID: */
                    let mut id = [0u8; 6];
                    stream_reader
                        .read_exact(&mut id)
                        .await
                        .expect("Cannot read message id");
                    //println!("Message id: {:?}", id);

                    /* Get message type: */
                    let mut message_type = [0u8; 4];
                    stream_reader
                        .read_exact(&mut message_type)
                        .await
                        .expect("Cannot read message type");
                    //println!("Message type: {:?}", message_type);

                    /* Get subscription id: */
                    let mut subscription_id = vec![0u8; 4];
                    stream_reader
                        .read_exact(&mut subscription_id)
                        .await
                        .expect("Cannot read subscription id");
                    //println!("Subscription id: {:?}", subscription_id.clone());

                    /* Get message: */
                    let (message, length): (Vec<u8>, u32) = stream_reader.read_message().await;
                    //println!("Length: {:?}", length);
                    //println!("Message: {:?}", message);

                    let mut subscriptions = reader.subscriptions.write().await;
                    match subscriptions.get_mut(&subscription_id) {
                        Some(subscriber) => {
                            //println!("Found subscription -> relaying");
                            let schemas = schemas.read().await;
                            let schema = schemas.get(&message_type[..]).unwrap();
                            let decoded_message = from_avro_datum(schema, &mut message.as_slice(), None).unwrap();
                            subscriber.send(decoded_message).await;
                            //println!("Subscription relayed");
                        }
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
    pub async fn expect_ack(&self, id: &[u8]) -> Result<bool, RecvError> {
        let (sender, receiver): (oneshot::Sender<bool>, oneshot::Receiver<bool>) =
            oneshot::channel();
        {
            let mut pending_acknowledgements = self.pending_acknowledgements.lock().await;
            pending_acknowledgements.insert(id.to_vec(), sender);
        }
        receiver.await
    }
}
