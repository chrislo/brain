use crate::event::Event;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct StepSequencer {
    steps: HashSet<Step>,
    pub active_note_number: i32,
}

#[derive(Clone, Copy, Debug, Hash, Eq)]
struct Step {
    tick: i32,
    note_number: i32,
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.tick == other.tick && self.note_number == other.note_number
    }
}

impl StepSequencer {
    pub fn empty() -> StepSequencer {
        StepSequencer {
            steps: HashSet::new(),
            active_note_number: 1,
        }
    }

    pub fn events_for_tick(&self, tick: i32) -> Vec<Event> {
        let track_length_in_ticks = 96;
        let offset_into_track = tick % track_length_in_ticks;

        self.steps
            .clone()
            .into_iter()
            .filter(|s| offset_into_track == s.tick)
            .map(|s| Event {
                note_number: s.note_number,
            })
            .collect()
    }

    pub fn toggle_sixteenth(&self, sixteenth: i32) -> StepSequencer {
        let step = Step {
            tick: (sixteenth - 1) * 6,
            note_number: self.active_note_number,
        };
        self.toggle_step(step)
    }

    fn toggle_step(&self, step: Step) -> StepSequencer {
        let mut steps = self.steps.clone();

        if self.steps.contains(&step) {
            steps.remove(&step);
        } else {
            steps.insert(step);
        }
        StepSequencer {
            steps: steps,
            active_note_number: self.active_note_number,
        }
    }

    pub fn active_sixteenths(&self) -> HashSet<i32> {
        let mut steps = self.steps.clone();
        steps.retain(|&s| s.note_number == self.active_note_number);
        steps.into_iter().map(|s| (s.tick / 6) + 1).collect()
    }

    pub fn increment_active_note_number(&self) -> StepSequencer {
        StepSequencer {
            steps: self.steps.clone(),
            active_note_number: self.active_note_number + 1,
        }
    }

    pub fn decrement_active_note_number(&self) -> StepSequencer {
        StepSequencer {
            steps: self.steps.clone(),
            active_note_number: self.active_note_number - 1,
        }
    }
}

#[test]
fn test_events_for_tick() {
    let track = StepSequencer::empty()
        .toggle_sixteenth(1)
        .toggle_sixteenth(5)
        .toggle_sixteenth(9)
        .toggle_sixteenth(13);

    for n in 0..96 {
        let events = track.events_for_tick(n);
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

    let track = StepSequencer::empty()
        .toggle_sixteenth(1)
        .increment_active_note_number()
        .toggle_sixteenth(1);
    let events = track.events_for_tick(0);
    assert_eq!(2, events.len());
}

#[test]
fn test_toggle_sixteenth() {
    let track = StepSequencer::empty();

    let processed_track = track.toggle_sixteenth(2);
    assert_eq!(1, processed_track.active_sixteenths().len());

    let processed_track = processed_track.toggle_sixteenth(2);
    assert_eq!(0, processed_track.active_sixteenths().len());
}

#[test]
fn test_toggle_sixteenth_with_different_note_numbers() {
    let track = StepSequencer::empty();

    assert_eq!(1, track.toggle_sixteenth(2).active_sixteenths().len());

    assert_eq!(
        1,
        track
            .toggle_sixteenth(1)
            .increment_active_note_number()
            .toggle_sixteenth(2)
            .active_sixteenths()
            .len()
    );

    assert_eq!(
        1,
        track
            .toggle_sixteenth(1)
            .increment_active_note_number()
            .toggle_sixteenth(2)
            .decrement_active_note_number()
            .active_sixteenths()
            .len()
    );
}

#[test]
fn test_step_equality() {
    let step_1 = Step {
        tick: 0,
        note_number: 1,
    };
    let step_2 = Step {
        tick: 0,
        note_number: 2,
    };
    let step_3 = Step {
        tick: 6,
        note_number: 1,
    };
    assert!(step_1 == step_1);
    assert!(step_1 != step_2);
    assert!(step_1 != step_3);
}

#[test]
fn test_active_sixteenths_with_note_number() {
    let active_sixteenths = StepSequencer::empty()
        .toggle_sixteenth(1)
        .increment_active_note_number()
        .toggle_sixteenth(16)
        .active_sixteenths();

    assert_eq!(1, active_sixteenths.len());
    assert!(active_sixteenths.contains(&16));
}