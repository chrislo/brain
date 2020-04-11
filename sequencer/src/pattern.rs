use crate::measure::Measure;

pub struct Pattern {
    notes: Vec<Measure>,
}

impl Pattern {
    pub fn new(notes: Vec<Measure>) -> Pattern {
        Pattern { notes: notes }
    }

    pub fn notes_between(&self, start: Measure, end: Measure) -> Vec<Measure> {
        let start_float = start.reduce_to_one_bar().to_float();
        let end_float = end.reduce_to_one_bar().to_float();

        self.notes
            .clone()
            .into_iter()
            .filter(|n| n.to_float() > start_float && n.to_float() <= end_float)
            .collect::<Vec<Measure>>()
    }
}

#[test]
fn test_notes_between() {
    let pattern = Pattern::new(vec![Measure(2, 16)]);

    let notes = pattern.notes_between(Measure(1, 16), Measure(3, 16));
    assert_eq!(Measure(2, 16), notes[0]);

    let notes = pattern.notes_between(Measure(3, 16), Measure(4, 16));
    assert!(notes.is_empty());

    let notes = pattern.notes_between(Measure(17, 16), Measure(19, 16));
    assert_eq!(Measure(2, 16), notes[0]);

    let pattern = Pattern::new(vec![Measure(1, 16)]);
    let notes = pattern.notes_between(Measure(1, 32), Measure(2, 32));
    assert_eq!(Measure(1, 16), notes[0]);
}
