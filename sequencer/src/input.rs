use rosc::OscPacket;
use std::net::UdpSocket;

#[derive(Debug, Clone)]
pub enum Message {
    NoteOn { note_number: i32 },
    NoteOff { note_number: i32 },
    KnobIncrement { number: i32 },
    KnobDecrement { number: i32 },
    Left,
    Right,
    SelectOn,
    SelectOff,
    Up,
    ShiftOn,
    ShiftOff,
    Unhandled,
}

pub fn process_incoming_message(sock: &UdpSocket) -> Option<Message> {
    let mut buf = [0u8; rosc::decoder::MTU];

    match sock.recv_from(&mut buf) {
        Ok((size, _addr)) => {
            let packet = rosc::decoder::decode(&buf[..size]).unwrap();
            let message = parse_incoming_osc_message(packet);
            match message {
                Message::Unhandled => None,
                _ => Some(message),
            }
        }
        Err(e) => {
            println!("Error receiving from socket: {}", e);
            None
        }
    }
}

fn parse_incoming_osc_message(packet: OscPacket) -> Message {
    match packet {
        OscPacket::Message(msg) => {
            if msg.addr.contains("note_on") {
                match msg.args[0] {
                    rosc::OscType::Int(i) => Message::NoteOn { note_number: i },
                    _ => Message::Unhandled,
                }
            } else if msg.addr.contains("note_off") {
                match msg.args[0] {
                    rosc::OscType::Int(i) => Message::NoteOff { note_number: i },
                    _ => Message::Unhandled,
                }
            } else if msg.addr.contains("control_change") {
                match msg.args.as_slice() {
                    [rosc::OscType::Int(c), rosc::OscType::Int(v)] => {
                        if *c == 90 && *v == 127 {
                            Message::Left
                        } else if *c == 102 && *v == 127 {
                            Message::Right
                        } else if *c == 87 && *v == 127 {
                            Message::Up
                        } else if *c == 103 && *v == 127 {
                            Message::SelectOn
                        } else if *c == 103 && *v == 0 {
                            Message::SelectOff
                        } else if *c == 32 && *v == 127 {
                            Message::ShiftOn
                        } else if *c == 32 && *v == 0 {
                            Message::ShiftOff
                        } else if *c >= 14 && *c <= 17 && *v == 1 {
                            Message::KnobIncrement { number: c - 13 }
                        } else if *c >= 14 && *c <= 17 && *v == 65 {
                            Message::KnobDecrement { number: c - 13 }
                        } else {
                            Message::Unhandled
                        }
                    }
                    _ => Message::Unhandled,
                }
            } else {
                Message::Unhandled
            }
        }
        _ => Message::Unhandled,
    }
}

#[cfg(test)]
use rosc::OscMessage;

#[test]
fn test_parse_incoming_note_on_message() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/midi/atom/1/10/note_on".to_string(),
        args: vec![rosc::OscType::Int(36)],
    });
    let msg = parse_incoming_osc_message(packet);
    assert!(matches!(msg, Message::NoteOn { note_number: 36 }));
}

#[test]
fn test_parse_incoming_note_off_message() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/midi/atom/1/10/note_off".to_string(),
        args: vec![rosc::OscType::Int(36)],
    });
    let msg = parse_incoming_osc_message(packet);
    assert!(matches!(msg, Message::NoteOff { note_number: 36 }));
}

#[test]
fn test_parse_incoming_left_message() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/midi/atom/1/1/control_change".to_string(),
        args: vec![rosc::OscType::Int(90), rosc::OscType::Int(127)],
    });
    let msg = parse_incoming_osc_message(packet);
    assert!(matches!(msg, Message::Left));
}

#[test]
fn test_parse_incoming_right_message() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/midi/atom/1/1/control_change".to_string(),
        args: vec![rosc::OscType::Int(102), rosc::OscType::Int(127)],
    });
    let msg = parse_incoming_osc_message(packet);
    assert!(matches!(msg, Message::Right));
}

#[test]
fn test_parse_incoming_select_message() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/midi/atom/1/1/control_change".to_string(),
        args: vec![rosc::OscType::Int(103), rosc::OscType::Int(127)],
    });
    let msg = parse_incoming_osc_message(packet);
    assert!(matches!(msg, Message::SelectOn));

    let packet = OscPacket::Message(OscMessage {
        addr: "/midi/atom/1/1/control_change".to_string(),
        args: vec![rosc::OscType::Int(103), rosc::OscType::Int(0)],
    });
    let msg = parse_incoming_osc_message(packet);
    assert!(matches!(msg, Message::SelectOff));
}

#[test]
fn test_parse_incoming_up_message() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/midi/atom/1/1/control_change".to_string(),
        args: vec![rosc::OscType::Int(87), rosc::OscType::Int(127)],
    });
    let msg = parse_incoming_osc_message(packet);
    assert!(matches!(msg, Message::Up));
}

#[test]
fn test_parse_incoming_shift_on_message() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/midi/atom/1/1/control_change".to_string(),
        args: vec![rosc::OscType::Int(32), rosc::OscType::Int(127)],
    });
    let msg = parse_incoming_osc_message(packet);
    assert!(matches!(msg, Message::ShiftOn));
}

#[test]
fn test_parse_incoming_shift_off_message() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/midi/atom/1/1/control_change".to_string(),
        args: vec![rosc::OscType::Int(32), rosc::OscType::Int(0)],
    });
    let msg = parse_incoming_osc_message(packet);
    assert!(matches!(msg, Message::ShiftOff));
}

#[test]
fn test_parse_incoming_knob_control_change() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/midi/atom/1/1/control_change".to_string(),
        args: vec![rosc::OscType::Int(14), rosc::OscType::Int(65)],
    });
    let msg = parse_incoming_osc_message(packet);

    assert!(matches!(msg, Message::KnobDecrement { number: 1 }));

    let packet = OscPacket::Message(OscMessage {
        addr: "/midi/atom/1/1/control_change".to_string(),
        args: vec![rosc::OscType::Int(15), rosc::OscType::Int(65)],
    });
    let msg = parse_incoming_osc_message(packet);
    assert!(matches!(msg, Message::KnobDecrement { number: 2 }));

    let packet = OscPacket::Message(OscMessage {
        addr: "/midi/atom/1/1/control_change".to_string(),
        args: vec![rosc::OscType::Int(15), rosc::OscType::Int(1)],
    });
    let msg = parse_incoming_osc_message(packet);
    assert!(matches!(msg, Message::KnobIncrement { number: 2 }));
}
