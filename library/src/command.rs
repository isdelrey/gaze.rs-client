use std::convert::TryFrom;

#[derive(Debug)]
#[repr(u8)]
pub enum Command {
    Message = 0x00,
    SubscriptionMessage = 0x01,
    Subscription = 0x02,
    MessageAck = 0x03,
    MessageNack = 0x04,
    Schema = 0x08,
    SchemaOffer = 0x09,
    SchemaNeeded = 0x10,
    NoSchema = 0x11,
}

impl TryFrom<u8> for Command {
    type Error = &'static str;
    fn try_from(byte: u8) -> Result<Self, &'static str> {
        match byte {
            0x00 => Ok(Command::Message),
            0x01 => Ok(Command::SubscriptionMessage),
            0x02 => Ok(Command::Subscription),
            0x03 => Ok(Command::MessageAck),
            0x04 => Ok(Command::MessageNack),
            0x08 => Ok(Command::Schema),
            0x09 => Ok(Command::SchemaOffer),
            0x10 => Ok(Command::SchemaNeeded),
            0x11 => Ok(Command::NoSchema),
            _ => Err("Cannot convert u8 to Command: byte not valid"),
        }
    }
}
