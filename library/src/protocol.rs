use crate::command::Command;
use crate::numbers::VarIntEncoder;
use async_trait::async_trait;
use std::convert::TryFrom;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

#[async_trait]
pub trait ReadProtocol {
    async fn read_command(&mut self) -> Result<Command, ()>;
    async fn read_ack(&mut self) -> Vec<u8>;
    async fn read_message(&mut self) -> (Vec<u8>, u32);
}

#[async_trait]
impl ReadProtocol for OwnedReadHalf {
    async fn read_command(&mut self) -> Result<Command, ()> {
        let mut command = [0u8; 1];

        //println!("Reading command");
        match self.read_exact(&mut command).await {
            Ok(_) => {
                //println!("Read {:?}", command[0]);
                let command = Command::try_from(command[0]).unwrap();
                //println!("Read {:?}", command);
                Ok(command)
            },
            Err(e) => {
                //println!("Error reading command: {:?}", e);
                Err(())
            },
        }
    }

    async fn read_ack(&mut self) -> Vec<u8> {
        let mut received_id: Vec<u8> = [0u8; 6].to_vec();
        self.read_exact(&mut received_id).await.unwrap();

        received_id
    }

    async fn read_message(&mut self) -> (Vec<u8>, u32) {
        let mut raw_length = [0u8; 4];
        self.read_exact(&mut raw_length).await.unwrap();
        let length = u32::from_le_bytes(raw_length);
        let mut message = vec![0u8; length as usize];
        self.read_exact(&mut message).await.unwrap();

        (message, length)
    }
}

#[async_trait]
pub trait WriteProtocol {
    async fn write_command(&mut self, command: Command);
    async fn write_size(&mut self, size: usize);
    async fn write_message_ack(&mut self, id: Vec<u8>);
    async fn write_message_nack(&mut self, id: Vec<u8>);
    async fn write_id(&mut self, id: &[u8]);
}

#[async_trait]
impl WriteProtocol for OwnedWriteHalf {
    async fn write_size(&mut self, size: usize) {
        let bytes = &(size as u32).to_le_bytes();
        //println!("Size is {} -> {:?}", size, bytes);

        self.write(bytes).await.unwrap();
    }

    async fn write_command(&mut self, command: Command) {
        //println!("Writing {:?}", command);
        self.write(&[command as u8]).await.unwrap();
    }

    async fn write_message_ack(&mut self, id: Vec<u8>) {
        self.write_command(Command::MessageAck).await;
        self.write(&id).await.unwrap();
    }

    async fn write_message_nack(&mut self, id: Vec<u8>) {
        self.write_command(Command::MessageNack).await;
        self.write(&id).await.unwrap();
    }

    async fn write_id(&mut self, id: &[u8]) {
        self.write(id).await.unwrap();
    }
}
