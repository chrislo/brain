use rosc::encoder;
use rosc::{OscMessage, OscPacket};
use std::net::SocketAddrV4;
use std::net::UdpSocket;
use std::str::FromStr;

pub fn send_via_osc(packet: Vec<u8>) {
    let sock = UdpSocket::bind("0.0.0.0:0").unwrap();
    let to_addr = SocketAddrV4::from_str("127.0.0.1:49162").unwrap();

    sock.send_to(&packet, to_addr).unwrap();
}

pub fn send_osc_to_o2m(packet: Vec<u8>) {
    let sock = UdpSocket::bind("0.0.0.0:0").unwrap();
    let to_addr = SocketAddrV4::from_str("127.0.0.1:57200").unwrap();

    sock.send_to(&packet, to_addr).unwrap();
}

pub fn send_clock() {
    let packet = encoder::encode(&OscPacket::Message(OscMessage {
        addr: "/*/clock".to_string(),
        args: vec![],
    }))
    .unwrap();

    send_osc_to_o2m(packet);
}
