use crate::event::Event;
use crate::measure::Measure;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Track {
    steps: HashSet<Step>,
}

#[derive(Clone, Copy, Debug, Hash, Eq)]
struct Step {
    measure: Measure,
    note_number: i32,
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.measure == other.measure && self.note_number == other.note_number
    }
}

impl Track {
    pub fn empty() -> Track {
        Track {
            steps: HashSet::new(),
        }
    }

    pub fn events_between(&self, start: Measure, end: Measure) -> Vec<Event> {
        let start_float = start.reduce_to_one_bar().to_float();
        let end_float = end.reduce_to_one_bar().to_float();

        self.steps
            .clone()
            .into_iter()
            .map(|s| Event {
                start: s.measure,
                note_number: s.note_number,
            })
            .filter(|e| e.start.to_float() > start_float && e.start.to_float() <= end_float)
            .collect::<Vec<Event>>()
    }

    pub fn toggle_sixteenth(&self, sixteenth: i32, note_number: i32) -> Track {
        let step = Step {
            measure: Measure(sixteenth, 16),
            note_number: note_number,
        };
        self.toggle_step(step)
    }

    fn toggle_step(&self, step: Step) -> Track {
        let mut steps = self.steps.clone();

        if self.steps.contains(&step) {
            steps.remove(&step);
        } else {
            steps.insert(step);
        }
        Track { steps: steps }
    }

    pub fn active_sixteenths_with_note_number(&self, note_number: i32) -> HashSet<i32> {
        let mut steps = self.steps.clone();
        steps.retain(|&s| s.note_number == note_number);
        steps.into_iter().map(|s| s.measure.0).collect()
    }
}

#[test]
fn test_events_between() {
    let track = Track::empty().toggle_sixteenth(2, 1);

    let events = track.events_between(Measure(1, 16), Measure(3, 16));
    assert_eq!(Measure(2, 16), events[0].start);

    let events = track.events_between(Measure(3, 16), Measure(4, 16));
    assert!(events.is_empty());

    let events = track.events_between(Measure(17, 16), Measure(19, 16));
    assert_eq!(Measure(2, 16), events[0].start);

    let track = Track::empty().toggle_sixteenth(1, 1);
    let events = track.events_between(Measure(1, 32), Measure(2, 32));
    assert_eq!(Measure(1, 16), events[0].start);
}

#[test]
fn test_toggle_sixteenth() {
    let track = Track::empty();

    let processed_track = track.toggle_sixteenth(2, 1);
    assert_eq!(
        1,
        processed_track.active_sixteenths_with_note_number(1).len()
    );

    let processed_track = processed_track.toggle_sixteenth(2, 1);
    assert_eq!(
        0,
        processed_track.active_sixteenths_with_note_number(1).len()
    );
}

#[test]
fn test_toggle_sixteenth_with_different_note_numbers() {
    let track = Track::empty();

    let processed_track = track.toggle_sixteenth(2, 1);
    assert_eq!(
        1,
        processed_track.active_sixteenths_with_note_number(1).len()
    );

    let processed_track = processed_track.toggle_sixteenth(2, 2);
    assert_eq!(
        1,
        processed_track.active_sixteenths_with_note_number(1).len()
    );
    assert_eq!(
        1,
        processed_track.active_sixteenths_with_note_number(2).len()
    );
}

#[test]
fn test_step_equality() {
    let step_1 = Step {
        measure: Measure(1, 16),
        note_number: 1,
    };
    let step_2 = Step {
        measure: Measure(1, 16),
        note_number: 2,
    };
    let step_3 = Step {
        measure: Measure(2, 16),
        note_number: 1,
    };
    assert!(step_1 == step_1);
    assert!(step_1 != step_2);
    assert!(step_1 != step_3);
}

#[test]
fn test_active_sixteenths_with_note_number() {
    let step_1 = Step {
        measure: Measure(1, 16),
        note_number: 1,
    };
    let step_2 = Step {
        measure: Measure(16, 16),
        note_number: 2,
    };
    let active_sixteenths = Track::empty()
        .toggle_step(step_1)
        .toggle_step(step_2)
        .active_sixteenths_with_note_number(2);

    assert_eq!(1, active_sixteenths.len());
    assert!(active_sixteenths.contains(&16));
}
