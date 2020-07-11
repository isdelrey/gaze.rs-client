use std::convert::TryFrom;

#[derive(Debug)]
#[repr(u8)]
pub enum Command {
    Message = 0x01,
    SubscriptionMessage = 0x02,
    Subscribe = 0x03,
    Unsubscribe = 0x04,
    MessageAck = 0x05,
    MessageNack = 0x06,
    Schema = 0x07,
    NoSchema = 0x08,
}

impl TryFrom<u8> for Command {
    type Error = &'static str;
    fn try_from(byte: u8) -> Result<Self, &'static str> {
        match byte {
            0x01 => Ok(Command::Message),
            0x02 => Ok(Command::SubscriptionMessage),
            0x03 => Ok(Command::Subscribe),
            0x04 => Ok(Command::Unsubscribe),
            0x05 => Ok(Command::MessageAck),
            0x06 => Ok(Command::MessageNack),
            0x07 => Ok(Command::Schema),
            0x08 => Ok(Command::NoSchema),
            _ => {
                //println!("Received unmappable command {}", byte);
                Err("Cannot convert u8 to Command: byte not valid")
            },
        }
    }
}
