use crate::event::Event;
use crate::measure::Measure;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Track {
    steps: HashSet<Step>,
}

#[derive(Clone, Copy, Hash, Eq)]
pub struct Step {
    pub measure: Measure,
    pub note_number: i32,
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
            .map(|s| Event { start: s.measure })
            .filter(|e| e.start.to_float() > start_float && e.start.to_float() <= end_float)
            .collect::<Vec<Event>>()
    }

    pub fn toggle_step(&self, step: Step) -> Track {
        let mut steps = self.steps.clone();

        if self.steps.contains(&step) {
            steps.remove(&step);
        } else {
            steps.insert(step);
        }
        Track { steps: steps }
    }

    pub fn active_steps(&self) -> HashSet<Step> {
        self.steps.clone()
    }
}

#[test]
fn test_events_between() {
    let track = Track::empty().toggle_step(Step {
        measure: Measure(2, 16),
        note_number: 1,
    });

    let events = track.events_between(Measure(1, 16), Measure(3, 16));
    assert_eq!(Measure(2, 16), events[0].start);

    let events = track.events_between(Measure(3, 16), Measure(4, 16));
    assert!(events.is_empty());

    let events = track.events_between(Measure(17, 16), Measure(19, 16));
    assert_eq!(Measure(2, 16), events[0].start);

    let track = Track::empty().toggle_step(Step {
        measure: Measure(1, 16),
        note_number: 1,
    });
    let events = track.events_between(Measure(1, 32), Measure(2, 32));
    assert_eq!(Measure(1, 16), events[0].start);
}

#[test]
fn test_active_steps() {
    assert_eq!(0, Track::empty().active_steps().len());

    let step_1 = Step {
        measure: Measure(1, 16),
        note_number: 1,
    };
    let step_2 = Step {
        measure: Measure(16, 16),
        note_number: 1,
    };
    let active_steps = Track::empty()
        .toggle_step(step_1)
        .toggle_step(step_2)
        .active_steps();

    assert_eq!(2, active_steps.len());
    assert!(active_steps.contains(&step_1));
    assert!(active_steps.contains(&step_2));
}

#[test]
fn test_toggle_step() {
    let track = Track::empty();

    let processed_track = track.toggle_step(Step {
        measure: Measure(2, 16),
        note_number: 1,
    });
    assert_eq!(1, processed_track.active_steps().len());

    let processed_track = processed_track.toggle_step(Step {
        measure: Measure(2, 16),
        note_number: 1,
    });
    assert_eq!(0, processed_track.active_steps().len());
}

#[test]
fn test_toggle_step_with_different_note_numbers() {
    let track = Track::empty();

    let processed_track = track.toggle_step(Step {
        measure: Measure(2, 16),
        note_number: 1,
    });
    assert_eq!(1, processed_track.active_steps().len());

    let processed_track = processed_track.toggle_step(Step {
        measure: Measure(2, 16),
        note_number: 2,
    });
    assert_eq!(2, processed_track.active_steps().len());
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
