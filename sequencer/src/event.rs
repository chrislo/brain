use rosc::OscMessage;

#[derive(Clone, Copy, Debug, Eq)]
pub struct Event {
    pub note_number: i32,
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.note_number == other.note_number
    }
}

impl Event {
    pub fn to_osc_message(&self) -> OscMessage {
        OscMessage {
            addr: "/sampler".to_string(),
            args: vec![rosc::OscType::Int(self.note_number)],
        }
    }
}
