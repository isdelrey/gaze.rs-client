use async_trait::async_trait;
use crate::numbers::VarIntEncoder;
use crate::command::Command;
use tokio::io::{AsyncReadExt,AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use std::convert::TryFrom;

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

        match self.read_exact(&mut command).await {
            Ok(_) => Ok(Command::try_from(command[0]).unwrap()),
            Err(_) => Err(())

        }
    }

    async fn read_ack(&mut self) -> Vec<u8> {
        let mut received_id: Vec<u8> = [0u8; 10].to_vec();
        self.read_exact(&mut received_id).await.unwrap();

        received_id
    }


    async fn read_message(&mut self) -> (Vec<u8>, u32) {
        let mut raw_length  = [0u8; 4];
        self.read_exact(&mut raw_length).await.unwrap();
        let length = u32::from_le_bytes(raw_length);
        
        let message = vec![0u8; length as usize];

        (message, length)
    }
}


#[async_trait]
pub trait WriteProtocol {
    async fn write_command(&mut self, command: Command);
    async fn write_size(&mut self, size: usize);
    async fn write_ack(&mut self, id: Vec<u8>);
    async fn write_nack(&mut self, id: Vec<u8>);
    async fn write_id(&mut self, id: &[u8]);
}

#[async_trait]
impl WriteProtocol for OwnedWriteHalf {
    async fn write_size(&mut self, size: usize) {
        println!("Size is: {}", size);

        self.write(&size.to_le_bytes()).await.unwrap();
    }

    async fn write_command(&mut self, command: Command) {
        self.write(&[command as u8]).await.unwrap();
    }

    async fn write_ack(&mut self, id: Vec<u8>) {
        self.write_command(Command::Ack).await;
        self.write(&id).await.unwrap();
    }

    async fn write_nack(&mut self, id: Vec<u8>) {
        self.write_command(Command::Nack).await;
        self.write(&id).await.unwrap();
    }

    async fn write_id(&mut self, id: &[u8]) {
        self.write(id).await.unwrap();
    }
}