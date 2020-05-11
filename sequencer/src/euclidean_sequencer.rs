use crate::event::Event;
use std::collections::HashMap;

#[derive(Debug)]
pub struct EuclideanSequencer {
    patterns: HashMap<i32, Pattern>,
}

impl EuclideanSequencer {
    pub fn empty() -> EuclideanSequencer {
        EuclideanSequencer {
            patterns: HashMap::new(),
        }
    }

    pub fn events_for_tick(&self, tick: i32) -> Vec<Event> {
        let mut events = vec![];
        let ticks_per_sixteenth = 96 / 16 as i32;

        for (note_number, pattern) in self.patterns.iter() {
            let pattern_length_in_ticks = ticks_per_sixteenth * pattern.pulses as i32;
            let offset_into_pattern = tick % pattern_length_in_ticks;
            let euclidean_pattern = euclidean_pattern(*pattern);

            if offset_into_pattern % ticks_per_sixteenth == 0 {
                let idx = offset_into_pattern / ticks_per_sixteenth;
                if euclidean_pattern[idx as usize] == 1 {
                    events.push(Event {
                        note_number: *note_number,
                    })
                }
            }
        }

        events
    }

    pub fn add_pattern(&self, note_number: i32, pattern: &Pattern) -> EuclideanSequencer {
        let mut patterns = self.patterns.clone();
        patterns.insert(note_number, *pattern);
        EuclideanSequencer { patterns: patterns }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pattern {
    onsets: usize,
    pulses: usize,
    rotate: usize,
}

pub fn euclidean_pattern(pattern: Pattern) -> Vec<i32> {
    let slope = pattern.onsets as f32 / pattern.pulses as f32;
    let mut previous = 1;
    let mut result = vec![0; pattern.pulses];

    if pattern.onsets == 0 {
        return result;
    }

    for i in 0..pattern.pulses {
        let current = (i as f32 * slope).floor() as i32;
        if current != previous {
            result[i] = 1;
        }
        previous = current;
    }

    result.rotate_right(pattern.rotate);
    result
}

#[test]
fn test_events_for_tick_with_single_pattern() {
    let pattern = Pattern {
        onsets: 1,
        pulses: 2,
        rotate: 0,
    };

    assert_eq!(euclidean_pattern(pattern), vec!(1, 0));

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

    assert_eq!(euclidean_pattern(pattern1), vec!(1, 0));
    assert_eq!(euclidean_pattern(pattern2), vec!(0, 1));

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
fn test_euclidean_pattern() {
    let pattern = euclidean_pattern(Pattern {
        onsets: 4,
        pulses: 16,
        rotate: 0,
    });
    assert_eq!(
        pattern,
        vec!(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0)
    );

    let pattern = euclidean_pattern(Pattern {
        onsets: 5,
        pulses: 12,
        rotate: 0,
    });
    assert_eq!(pattern, vec!(1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0));

    let pattern = euclidean_pattern(Pattern {
        onsets: 5,
        pulses: 12,
        rotate: 1,
    });
    assert_eq!(pattern, vec!(0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1));

    let pattern = euclidean_pattern(Pattern {
        onsets: 0,
        pulses: 4,
        rotate: 0,
    });
    assert_eq!(pattern, vec!(0, 0, 0, 0));

    let pattern = euclidean_pattern(Pattern {
        onsets: 4,
        pulses: 4,
        rotate: 0,
    });
    assert_eq!(pattern, vec!(1, 1, 1, 1));
}
