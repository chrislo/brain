use crate::measure::Measure;
use rosc::OscPacket;

pub fn parse_incoming_osc_message(packet: OscPacket) -> Result<Measure, &'static str> {
    match packet {
        OscPacket::Message(msg) => match msg.args[0] {
            rosc::OscType::Int(i) => Ok(Measure(i - 35, 16)),
            _ => Err("Unable to handle packet"),
        },
        _ => Err("Unable to handle packet"),
    }
}

#[cfg(test)]
use rosc::OscMessage;

#[test]
fn test_parse_incoming_osc_message() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/midi/atom/1/10/note_on".to_string(),
        args: vec![rosc::OscType::Int(36)],
    });
    let msg = parse_incoming_osc_message(packet);
    assert_eq!(msg.unwrap(), Measure(1, 16));

    let packet = OscPacket::Message(OscMessage {
        addr: "/midi/atom/1/10/note_on".to_string(),
        args: vec![rosc::OscType::Int(51)],
    });
    let msg = parse_incoming_osc_message(packet);
    assert_eq!(msg.unwrap(), Measure(16, 16));
}
