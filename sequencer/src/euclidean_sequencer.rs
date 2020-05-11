use std::collections::HashMap;

#[derive(Debug)]
pub struct EuclideanSequencer {
    pub patterns: HashMap<i32, Pattern>,
}

impl EuclideanSequencer {
    pub fn empty() -> EuclideanSequencer {
        EuclideanSequencer {
            patterns: HashMap::new(),
        }
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
fn test_track_add_pattern() {
    let track = EuclideanSequencer::empty();
    let pattern = Pattern {
        onsets: 4,
        pulses: 16,
        rotate: 0,
    };

    let new_track = track.add_pattern(1, &pattern);
    assert_eq!(Some(&pattern), new_track.patterns.get(&1))
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
