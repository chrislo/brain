use rosc::encoder;
use rosc::{OscMessage, OscPacket};
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;

pub fn handshake() {
    let packet = encoder::encode(&OscPacket::Message(OscMessage {
        addr: "/atom/note_off".to_string(),
        args: vec![
            rosc::OscType::Int(16),
            rosc::OscType::Int(0),
            rosc::OscType::Int(127),
        ],
    }))
    .unwrap();

    send_osc_to_o2m(packet);
}

fn send_osc_to_o2m(packet: Vec<u8>) {
    let sock = UdpSocket::bind("0.0.0.0:0").unwrap();
    let to_addr = SocketAddrV4::from_str("127.0.0.1:57200").unwrap();

    sock.send_to(&packet, to_addr).unwrap();
}
