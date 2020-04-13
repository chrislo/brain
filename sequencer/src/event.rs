use crate::measure::Measure;
use rosc::encoder;
use rosc::{OscMessage, OscPacket};
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Event {
    pub start: Measure,
}

impl Event {
    pub fn send_via_osc(&self) {
        let sock = UdpSocket::bind("0.0.0.0:0").unwrap();
        let to_addr = SocketAddrV4::from_str("127.0.0.1:49162").unwrap();

        sock.send_to(&self.to_osc_message(), to_addr).unwrap();
    }

    fn to_osc_message(&self) -> Vec<u8> {
        encoder::encode(&OscPacket::Message(OscMessage {
            addr: "/sampler/1".to_string(),
            args: vec![],
        }))
        .unwrap()
    }
}
