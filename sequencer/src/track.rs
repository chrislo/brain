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

    pub fn events_for_tick(&self, tick: Measure) -> Vec<Event> {
        let track_length_in_ticks = 96;
        let offset_into_track = tick.0 % track_length_in_ticks;

        self.steps
            .clone()
            .into_iter()
            .filter(|s| offset_into_track == (s.measure.0 * 6)) // FIXME: move this conversion from measure to tick offset somewhere else
            .map(|s| Event {
                note_number: s.note_number,
            })
            .collect()
    }

    pub fn toggle_sixteenth(&self, sixteenth: i32, note_number: i32) -> Track {
        let step = Step {
            measure: Measure(sixteenth - 1, 16),
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
fn test_events_for_tick() {
    let track = Track::empty()
        .toggle_sixteenth(1, 1)
        .toggle_sixteenth(5, 1)
        .toggle_sixteenth(9, 1)
        .toggle_sixteenth(13, 1);

    for n in 0..96 {
        let events = track.events_for_tick(Measure(n, 96));
        if n == 0 {
            assert_eq!(1, events.len());
        } else if n == 24 {
            assert_eq!(1, events.len());
        } else if n == 48 {
            assert_eq!(1, events.len());
        } else if n == 72 {
            assert_eq!(1, events.len());
        } else if n == 96 {
            assert_eq!(1, events.len());
        } else {
            assert!(events.is_empty());
        }
    }

    let track = Track::empty().toggle_sixteenth(1, 1).toggle_sixteenth(1, 2);
    let events = track.events_for_tick(Measure(0, 96));
    assert_eq!(2, events.len());
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
