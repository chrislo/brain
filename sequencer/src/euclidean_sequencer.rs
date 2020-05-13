use crate::event::Event;
use std::collections::HashMap;
use std::collections::HashSet;

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
        let current_pattern = self.current_or_default_pattern();

        patterns.insert(self.active_note_number, current_pattern.increment_onsets());

        EuclideanSequencer {
            patterns: patterns,
            active_note_number: self.active_note_number,
        }
    }

    pub fn decrement_onsets(&self) -> EuclideanSequencer {
        let mut patterns = self.patterns.clone();
        let current_pattern = self.current_or_default_pattern();

        patterns.insert(self.active_note_number, current_pattern.decrement_onsets());

        EuclideanSequencer {
            patterns: patterns,
            active_note_number: self.active_note_number,
        }
    }

    pub fn increment_pulses(&self) -> EuclideanSequencer {
        let mut patterns = self.patterns.clone();
        let current_pattern = self.current_or_default_pattern();

        patterns.insert(self.active_note_number, current_pattern.increment_pulses());

        EuclideanSequencer {
            patterns: patterns,
            active_note_number: self.active_note_number,
        }
    }

    pub fn decrement_pulses(&self) -> EuclideanSequencer {
        let mut patterns = self.patterns.clone();
        let current_pattern = self.current_or_default_pattern();

        patterns.insert(self.active_note_number, current_pattern.decrement_pulses());

        EuclideanSequencer {
            patterns: patterns,
            active_note_number: self.active_note_number,
        }
    }

    pub fn increment_rotate(&self) -> EuclideanSequencer {
        let mut patterns = self.patterns.clone();
        let current_pattern = self.current_or_default_pattern();

        patterns.insert(self.active_note_number, current_pattern.increment_rotate());

        EuclideanSequencer {
            patterns: patterns,
            active_note_number: self.active_note_number,
        }
    }

    pub fn decrement_rotate(&self) -> EuclideanSequencer {
        let mut patterns = self.patterns.clone();
        let current_pattern = self.current_or_default_pattern();

        patterns.insert(self.active_note_number, current_pattern.decrement_rotate());

        EuclideanSequencer {
            patterns: patterns,
            active_note_number: self.active_note_number,
        }
    }

    pub fn increment_active_note_number(&self) -> EuclideanSequencer {
        EuclideanSequencer {
            patterns: self.patterns.clone(),
            active_note_number: self.active_note_number + 1,
        }
    }

    pub fn decrement_active_note_number(&self) -> EuclideanSequencer {
        EuclideanSequencer {
            patterns: self.patterns.clone(),
            active_note_number: self.active_note_number - 1,
        }
    }

    fn current_or_default_pattern(&self) -> Pattern {
        if self.patterns.contains_key(&self.active_note_number) {
            *self.patterns.get(&self.active_note_number).unwrap()
        } else {
            Pattern::default()
        }
    }

    pub fn active_sixteenths(&self) -> HashSet<i32> {
        let pattern = self.current_or_default_pattern().euclidean_pattern();
        let mut active_sixteenths = HashSet::new();

        for (idx, v) in pattern.iter().enumerate() {
            if *v == 1 {
                active_sixteenths.insert((idx + 1) as i32);
            }
        }
        active_sixteenths
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

    pub fn decrement_onsets(&self) -> Pattern {
        Pattern {
            onsets: (self.onsets as i32 - 1).max(0) as usize,
            pulses: self.pulses,
            rotate: self.rotate,
        }
    }

    pub fn increment_pulses(&self) -> Pattern {
        Pattern {
            onsets: self.onsets,
            pulses: (self.pulses + 1).min(16),
            rotate: self.rotate,
        }
    }

    pub fn decrement_pulses(&self) -> Pattern {
        let new_pulses = self.pulses - 1;

        Pattern {
            onsets: self.onsets.min(new_pulses),
            pulses: new_pulses.max(1),
            rotate: self.rotate.min(new_pulses),
        }
    }

    pub fn increment_rotate(&self) -> Pattern {
        Pattern {
            onsets: self.onsets,
            pulses: self.pulses,
            rotate: (self.rotate + 1).min(self.pulses),
        }
    }

    pub fn decrement_rotate(&self) -> Pattern {
        Pattern {
            onsets: self.onsets,
            pulses: self.pulses,
            rotate: (self.rotate - 1).max(0),
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
fn test_active_sixteenths() {
    let pattern = Pattern {
        onsets: 1,
        pulses: 2,
        rotate: 0,
    };

    assert_eq!(pattern.euclidean_pattern(), vec!(1, 0));

    let active_sixteenths = EuclideanSequencer::empty()
        .add_pattern(1, &pattern)
        .active_sixteenths();

    assert_eq!(1, active_sixteenths.len());
    assert!(active_sixteenths.contains(&1));
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
fn test_decrement_onsets() {
    let sequencer = EuclideanSequencer::empty()
        .increment_onsets()
        .decrement_onsets();
    let events = sequencer.events_for_tick(0);
    assert_eq!(0, events.len());
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
fn test_pattern_decrement_onsets() {
    let pattern = Pattern {
        onsets: 1,
        pulses: 2,
        rotate: 0,
    };
    assert_eq!(0, pattern.decrement_onsets().onsets);

    // It prevents onsets being decremented lower than zero
    assert_eq!(0, pattern.decrement_onsets().decrement_onsets().onsets);
}

#[test]
fn test_pattern_decrement_pulses() {
    let pattern = Pattern {
        onsets: 1,
        pulses: 16,
        rotate: 0,
    };
    assert_eq!(15, pattern.decrement_pulses().pulses);

    // Decrement onsets and rotate to new pulses level if decrementing
    // pulses would make it smaller than the other two
    let pattern = Pattern {
        onsets: 3,
        pulses: 3,
        rotate: 3,
    };
    assert_eq!(2, pattern.decrement_pulses().onsets);
    assert_eq!(2, pattern.decrement_pulses().pulses);
    assert_eq!(2, pattern.decrement_pulses().rotate);
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
