use crate::measure::Measure;
use rosc::encoder;
use rosc::{OscMessage, OscPacket};

#[derive(Clone, Debug)]
pub struct Event {
    pub start: Measure,
}

impl Event {
    pub fn to_osc_message(&self) -> Vec<u8> {
        encoder::encode(&OscPacket::Message(OscMessage {
            addr: "/sampler/1".to_string(),
            args: vec![],
        }))
        .unwrap()
    }
}
