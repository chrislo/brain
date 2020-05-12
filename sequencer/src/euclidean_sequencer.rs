use crate::event::Event;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct EuclideanSequencer {
    patterns: HashMap<i32, Pattern>,
    active_note_number: i32,
}

impl EuclideanSequencer {
    pub fn empty() -> EuclideanSequencer {
        EuclideanSequencer {
            patterns: HashMap::new(),
            active_note_number: 1,
        }
    }

    pub fn events_for_tick(&self, tick: i32) -> Vec<Event> {
        let mut events = vec![];

        for (note_number, pattern) in self.patterns.iter() {
            if pattern.has_event_at_tick(tick) {
                events.push(Event {
                    note_number: *note_number,
                });
            }
        }

        events
    }

    pub fn add_pattern(&self, note_number: i32, pattern: &Pattern) -> EuclideanSequencer {
        let mut patterns = self.patterns.clone();
        patterns.insert(note_number, *pattern);
        EuclideanSequencer {
            patterns: patterns,
            active_note_number: self.active_note_number,
        }
    }

    pub fn increment_onsets(&self) -> EuclideanSequencer {
        let mut patterns = self.patterns.clone();
        let current_pattern = if patterns.contains_key(&self.active_note_number) {
            *patterns.get(&self.active_note_number).unwrap()
        } else {
            Pattern::default()
        };

        patterns.insert(self.active_note_number, current_pattern.increment_onsets());

        EuclideanSequencer {
            patterns: patterns,
            active_note_number: self.active_note_number,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pattern {
    onsets: usize,
    pulses: usize,
    rotate: usize,
}

impl Pattern {
    pub fn default() -> Pattern {
        Pattern {
            onsets: 0,
            pulses: 16,
            rotate: 0,
        }
    }

    pub fn increment_onsets(&self) -> Pattern {
        Pattern {
            onsets: (self.onsets + 1).min(self.pulses),
            pulses: self.pulses,
            rotate: self.rotate,
        }
    }

    fn euclidean_pattern(&self) -> Vec<i32> {
        let slope = self.onsets as f32 / self.pulses as f32;
        let mut previous = 1;
        let mut result = vec![0; self.pulses];

        if self.onsets == 0 {
            return result;
        }

        for i in 0..self.pulses {
            let current = (i as f32 * slope).floor() as i32;
            if current != previous {
                result[i] = 1;
            }
            previous = current;
        }

        result.rotate_right(self.rotate);
        result
    }

    pub fn has_event_at_tick(&self, tick: i32) -> bool {
        let ticks_per_sixteenth = 96 / 16 as i32;
        let pattern_length_in_ticks = ticks_per_sixteenth * self.pulses as i32;
        let offset_into_pattern = tick % pattern_length_in_ticks;

        let euclidean_pattern = self.euclidean_pattern();

        if offset_into_pattern % ticks_per_sixteenth == 0 {
            let idx = offset_into_pattern / ticks_per_sixteenth;
            euclidean_pattern[idx as usize] == 1
        } else {
            false
        }
    }
}

#[test]
fn test_events_for_tick_with_single_pattern() {
    let pattern = Pattern {
        onsets: 1,
        pulses: 2,
        rotate: 0,
    };

    assert_eq!(pattern.euclidean_pattern(), vec!(1, 0));

    let sequencer = EuclideanSequencer::empty().add_pattern(1, &pattern);

    for n in 0..13 {
        let events = sequencer.events_for_tick(n);
        if n == 0 || n == 12 {
            assert_eq!(1, events.len());
            assert_eq!(1, events[0].note_number);
        } else {
            assert!(events.is_empty());
        }
    }
}

#[test]
fn test_events_for_tick_with_multiple_patterns() {
    let pattern1 = Pattern {
        onsets: 1,
        pulses: 2,
        rotate: 0,
    };

    let pattern2 = Pattern {
        onsets: 1,
        pulses: 2,
        rotate: 1,
    };

    assert_eq!(pattern1.euclidean_pattern(), vec!(1, 0));
    assert_eq!(pattern2.euclidean_pattern(), vec!(0, 1));

    let sequencer = EuclideanSequencer::empty()
        .add_pattern(1, &pattern1)
        .add_pattern(2, &pattern2);

    for n in 0..13 {
        let events = sequencer.events_for_tick(n);
        if n == 0 || n == 12 {
            assert_eq!(1, events.len());
            assert_eq!(1, events[0].note_number);
        } else if n == 6 {
            assert_eq!(1, events.len());
            assert_eq!(2, events[0].note_number);
        } else {
            assert!(events.is_empty());
        }
    }
}

#[test]
fn test_increment_onsets() {
    let sequencer = EuclideanSequencer::empty().increment_onsets();
    let events = sequencer.events_for_tick(0);
    assert_eq!(1, events.len());
}

#[test]
fn test_pattern_increment_onsets() {
    let pattern = Pattern {
        onsets: 1,
        pulses: 2,
        rotate: 0,
    };
    assert_eq!(2, pattern.increment_onsets().onsets);

    // It prevents onsets being incremented larger than pulses
    assert_eq!(2, pattern.increment_onsets().increment_onsets().onsets);
}

#[test]
fn test_euclidean_pattern() {
    let pattern = Pattern {
        onsets: 4,
        pulses: 16,
        rotate: 0,
    };

    assert_eq!(
        pattern.euclidean_pattern(),
        vec!(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0)
    );

    let pattern = Pattern {
        onsets: 5,
        pulses: 12,
        rotate: 0,
    };
    assert_eq!(
        pattern.euclidean_pattern(),
        vec!(1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0)
    );

    let pattern = Pattern {
        onsets: 5,
        pulses: 12,
        rotate: 1,
    };
    assert_eq!(
        pattern.euclidean_pattern(),
        vec!(0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1)
    );

    let pattern = Pattern {
        onsets: 0,
        pulses: 4,
        rotate: 0,
    };
    assert_eq!(pattern.euclidean_pattern(), vec!(0, 0, 0, 0));

    let pattern = Pattern {
        onsets: 4,
        pulses: 4,
        rotate: 0,
    };
    assert_eq!(pattern.euclidean_pattern(), vec!(1, 1, 1, 1));
}

#[test]
fn test_pattern_has_events_at_tick() {
    let pattern = Pattern {
        onsets: 1,
        pulses: 2,
        rotate: 0,
    };

    assert_eq!(pattern.euclidean_pattern(), vec!(1, 0));

    assert!(pattern.has_event_at_tick(0));
    assert!(pattern.has_event_at_tick(12));

    for n in 1..12 {
        assert!(!pattern.has_event_at_tick(n));
    }
}
