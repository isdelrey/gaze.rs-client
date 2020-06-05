use std::convert::TryFrom;

#[derive(Debug)]
#[repr(u8)]
pub enum Command {
    Publish = 0x07, /* UTF8: BELL */
    Subscribe = 0x05, /* UTF8: ENQUIRY */
    Ack = 0x06, /* UTF8: ACK */
    Nack = 0x15, /* UTF8: NACK */
    AddModel = 0x16,
    GetModels = 0x17
}

impl TryFrom<u8> for Command {
    type Error = &'static str;
    fn try_from(byte: u8) -> Result<Self, &'static str> {
        match byte {
            0x07 => Ok(Command::Publish),
            0x05 => Ok(Command::Subscribe),
            0x06 => Ok(Command::Ack),
            0x15 => Ok(Command::Nack),
            0x16 => Ok(Command::AddModel),
            0x17 => Ok(Command::GetModels),
            _ => Err("Cannot convert u8 to Command: byte not valid")
        }
    }
}
