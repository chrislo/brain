use rosc::OscPacket;

#[derive(Debug, Clone)]
pub enum Message {
    NoteOn { note_number: i32 },
    NoteOff { note_number: i32 },
    KnobIncrement,
    KnobDecrement,
    Left,
    Right,
    Unhandled,
}

pub fn parse_incoming_osc_message(packet: OscPacket) -> Message {
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
                        } else if *c == 15 && *v == 1 {
                            Message::KnobIncrement
                        } else if *c == 15 && *v == 65 {
                            Message::KnobDecrement
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
fn test_parse_incoming_knob_control_change() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/midi/atom/1/1/control_change".to_string(),
        args: vec![rosc::OscType::Int(15), rosc::OscType::Int(65)],
    });
    let msg = parse_incoming_osc_message(packet);
    assert!(matches!(msg, Message::KnobDecrement));

    let packet = OscPacket::Message(OscMessage {
        addr: "/midi/atom/1/1/control_change".to_string(),
        args: vec![rosc::OscType::Int(15), rosc::OscType::Int(1)],
    });
    let msg = parse_incoming_osc_message(packet);
    assert!(matches!(msg, Message::KnobIncrement));
}
