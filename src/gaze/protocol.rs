use async_trait::async_trait;
use crate::gaze::numbers::VarIntEncoder;
use crate::gaze::command::Command;
use tokio::io::{AsyncReadExt,AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use std::convert::TryFrom;

#[async_trait]
pub trait ReadProtocol {
    async fn read_command(&mut self) -> Result<Command, ()>;
    async fn read_ack(&mut self) -> Vec<u8>;
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
        let size = size.encode_as_varint();
        println!("Encoded size is: {:?}", size);
        self.write(size.as_slice()).await.unwrap();
    }

    async fn write_command(&mut self, command: Command) {
        self.write(&[command as u8]).await.unwrap();
    }

    async fn write_ack(&mut self, id: Vec<u8>) {
        self.write_command(Command::Ack);
        self.write(&id).await.unwrap();
    }

    async fn write_nack(&mut self, id: Vec<u8>) {
        self.write_command(Command::Nack);
        self.write(&id).await.unwrap();
    }

    async fn write_id(&mut self, id: &[u8]) {
        self.write(id).await.unwrap();
    }
}