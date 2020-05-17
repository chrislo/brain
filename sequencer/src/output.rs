use rosc::encoder;
use rosc::{OscMessage, OscPacket};
use std::net::SocketAddrV4;
use std::net::UdpSocket;
use std::str::FromStr;

struct Output {
    socket: UdpSocket,
    to_addr: SocketAddrV4,
}

impl Output {
    pub fn sampler() -> Output {
        Output {
            socket: UdpSocket::bind("0.0.0.0:0").unwrap(),
            to_addr: SocketAddrV4::from_str("127.0.0.1:49162").unwrap(),
        }
    }

    pub fn o2m() -> Output {
        Output {
            socket: UdpSocket::bind("0.0.0.0:0").unwrap(),
            to_addr: SocketAddrV4::from_str("127.0.0.1:57200").unwrap(),
        }
    }

    pub fn send(&self, packet: Vec<u8>) {
        self.socket.send_to(&packet, self.to_addr).unwrap();
    }
}

pub fn send_osc_to_sampler(packet: Vec<u8>) {
    let output = Output::sampler();
    output.send(packet);
}

pub fn send_osc_to_o2m(packet: Vec<u8>) {
    let output = Output::o2m();
    output.send(packet);
}

pub fn send_clock() {
    let packet = encoder::encode(&OscPacket::Message(OscMessage {
        addr: "/*/clock".to_string(),
        args: vec![],
    }))
    .unwrap();

    send_osc_to_o2m(packet);
}
