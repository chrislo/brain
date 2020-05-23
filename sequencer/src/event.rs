use rosc::OscMessage;

#[derive(Clone, Copy, Debug)]
pub struct Event {
    pub note_number: i32,
}

impl Event {
    pub fn to_osc_message(&self) -> OscMessage {
        OscMessage {
            addr: "/sampler".to_string(),
            args: vec![rosc::OscType::Int(self.note_number)],
        }
    }
}
