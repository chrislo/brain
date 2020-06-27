use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Hash, Eq)]
pub struct Trigger {
    note_number: i32,
    offset: i32,
}

impl PartialEq for Trigger {
    fn eq(&self, other: &Self) -> bool {
        self.note_number == other.note_number && self.offset == other.offset
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq)]
pub struct Step(i32);

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug, Clone)]
pub struct Sequence {
    triggers: HashMap<Step, HashSet<Trigger>>,
    number_of_steps: i32,
}

impl Sequence {
    pub fn empty() -> Sequence {
        let mut triggers = HashMap::new();

        for n in 1..=16 {
            triggers.insert(Step(n), HashSet::new());
        }

        Sequence {
            triggers: triggers,
            number_of_steps: 16,
        }
    }

    pub fn triggers_for_tick(&self, tick: i32) -> Vec<Trigger> {
        let ticks_per_step = 6;
        let sequence_length_in_ticks = self.number_of_steps * ticks_per_step;
        let offset_into_sequence = tick % sequence_length_in_ticks;
        let nearest_step = Step((offset_into_sequence / ticks_per_step) + 1);
        let offset_into_step = offset_into_sequence % ticks_per_step;

        self.triggers
            .get(&nearest_step)
            .unwrap()
            .clone()
            .into_iter()
            .filter(|t| t.offset == offset_into_step)
            .collect()
    }

    pub fn trigger_note_number_at_step(&self, note_number: i32, step: Step) -> Sequence {
        let new_trigger = Trigger {
            note_number: note_number,
            offset: 0,
        };
        let mut triggers = self.triggers.clone();

        match self.triggers.get(&step) {
            Some(t) => {
                let mut step_triggers = t.clone();
                step_triggers.insert(new_trigger);
                triggers.insert(step, step_triggers);
            }
            None => {
                let mut step_triggers = HashSet::new();
                step_triggers.insert(new_trigger);
                triggers.insert(step, step_triggers);
            }
        }

        Sequence {
            triggers: triggers,
            ..self.clone()
        }
    }

    pub fn current_step(&self, tick: i32) -> Step {
        let ticks_per_step = 6;
        let sequence_length_in_ticks = self.number_of_steps * ticks_per_step;
        let offset_into_sequence = tick % sequence_length_in_ticks;

        Step((offset_into_sequence / ticks_per_step) + 1)
    }

    pub fn active_steps(&self) -> HashSet<Step> {
        let mut active_steps = HashSet::new();

        for (s, t) in self.triggers.iter() {
            if !t.is_empty() {
                active_steps.insert(*s);
            }
        }

        active_steps
    }

    pub fn set_length(&self, number_of_steps: i32) -> Sequence {
        let mut triggers = HashMap::new();

        for n in 1..=number_of_steps {
            let step = Step(n);
            match self.triggers.get(&step) {
                Some(t) => {
                    triggers.insert(step, t.clone());
                }
                None => {
                    triggers.insert(step, HashSet::new());
                }
            }
        }

        Sequence {
            triggers: triggers,
            number_of_steps: number_of_steps,
            ..self.clone()
        }
    }

    pub fn euclidean_fill(&self, note_number: i32, onsets: i32) -> Sequence {
        let slope = onsets as f32 / self.number_of_steps as f32;
        let mut previous = 1;
        let mut sequence = Sequence::empty().set_length(self.number_of_steps);

        if onsets > 0 {
            for i in 0..self.number_of_steps {
                let current = (i as f32 * slope).floor() as i32;
                if current != previous {
                    sequence = sequence.trigger_note_number_at_step(note_number, Step(i + 1));
                }
                previous = current;
            }
        }
        sequence
    }
}

#[test]
fn test_adding_trigger_to_sequence() {
    let sequence = Sequence::empty().trigger_note_number_at_step(1, Step(1));

    for n in 0..=96 {
        let triggers = sequence.triggers_for_tick(n);
        if n == 0 {
            assert_eq!(1, triggers.len());
        } else if n == 96 {
            assert_eq!(1, triggers.len());
        } else {
            assert!(triggers.is_empty());
        }
    }
}

#[test]
fn test_adding_two_triggers_to_sequence() {
    let sequence = Sequence::empty()
        .trigger_note_number_at_step(1, Step(1))
        .trigger_note_number_at_step(2, Step(1));

    let triggers = sequence.triggers_for_tick(0);
    assert_eq!(2, triggers.len());
}

#[test]
fn test_adding_the_same_trigger_twice_to_sequence() {
    let sequence = Sequence::empty()
        .trigger_note_number_at_step(1, Step(1))
        .trigger_note_number_at_step(1, Step(1));

    let triggers = sequence.triggers_for_tick(0);
    assert_eq!(1, triggers.len());
}

#[test]
fn test_current_step() {
    let sequencer = Sequence::empty();

    assert_eq!(Step(1), sequencer.current_step(0));
    assert_eq!(Step(1), sequencer.current_step(1));
    assert_eq!(Step(2), sequencer.current_step(6));
    assert_eq!(Step(16), sequencer.current_step(95));
    assert_eq!(Step(1), sequencer.current_step(96));
}

#[test]
fn test_active_steps() {
    let sequence = Sequence::empty()
        .trigger_note_number_at_step(1, Step(1))
        .trigger_note_number_at_step(1, Step(3));

    assert_eq!(2, sequence.active_steps().len());
    assert!(sequence.active_steps().contains(&Step(1)));
    assert!(sequence.active_steps().contains(&Step(3)));
}

#[test]
fn test_set_length_shorter() {
    let sequence = Sequence::empty()
        .trigger_note_number_at_step(1, Step(8))
        .trigger_note_number_at_step(1, Step(16));

    assert_eq!(2, sequence.active_steps().len());
    assert!(sequence.active_steps().contains(&Step(8)));
    assert!(sequence.active_steps().contains(&Step(16)));

    let shorter_sequence = sequence.set_length(8);
    assert_eq!(1, shorter_sequence.active_steps().len());
    assert!(shorter_sequence.active_steps().contains(&Step(8)));
    assert_eq!(8, shorter_sequence.number_of_steps);
}

#[test]
fn test_set_length_longer() {
    let sequence = Sequence::empty()
        .set_length(8)
        .trigger_note_number_at_step(1, Step(8));

    assert_eq!(1, sequence.active_steps().len());
    assert!(sequence.active_steps().contains(&Step(8)));
    assert_eq!(8, sequence.number_of_steps);

    let longer_sequence = sequence.set_length(16);
    assert_eq!(1, longer_sequence.active_steps().len());
    assert!(longer_sequence.active_steps().contains(&Step(8)));
    assert_eq!(16, longer_sequence.number_of_steps);

    // Check we can still iterate over sequence when made longer
    for n in 0..=96 {
        sequence.triggers_for_tick(n);
    }
}

#[test]
fn test_euclidean_fill() {
    let sequence = Sequence::empty().set_length(16).euclidean_fill(1, 4);
    assert_eq!(4, sequence.active_steps().len());
    assert!(sequence.active_steps().contains(&Step(1)));
    assert!(sequence.active_steps().contains(&Step(5)));
    assert!(sequence.active_steps().contains(&Step(9)));
    assert!(sequence.active_steps().contains(&Step(13)));

    let sequence = Sequence::empty().set_length(12).euclidean_fill(1, 5);
    assert_eq!(5, sequence.active_steps().len());
    assert!(sequence.active_steps().contains(&Step(1)));
    assert!(sequence.active_steps().contains(&Step(4)));
    assert!(sequence.active_steps().contains(&Step(6)));
    assert!(sequence.active_steps().contains(&Step(9)));
    assert!(sequence.active_steps().contains(&Step(11)));

    let sequence = Sequence::empty().set_length(2).euclidean_fill(1, 3);
    assert_eq!(2, sequence.active_steps().len());
    assert!(sequence.active_steps().contains(&Step(1)));
    assert!(sequence.active_steps().contains(&Step(2)));

    let sequence = Sequence::empty().set_length(2).euclidean_fill(1, 0);
    assert_eq!(0, sequence.active_steps().len());
}

#[test]
fn test_rotate() {
    let sequence = Sequence::empty()
        .set_length(3)
        .trigger_note_number_at_step(1, Step(1));

    assert_eq!(1, sequence.rotate(1).active_steps().len());
    assert!(sequence.rotate(1).active_steps().contains(&Step(2)));

    assert_eq!(1, sequence.rotate(2).active_steps().len());
    assert!(sequence.rotate(2).active_steps().contains(&Step(3)));

    assert_eq!(1, sequence.rotate(3).active_steps().len());
    assert!(sequence.rotate(3).active_steps().contains(&Step(1)));

    assert_eq!(1, sequence.rotate(-1).active_steps().len());
    assert!(sequence.rotate(-1).active_steps().contains(&Step(3)));
}
