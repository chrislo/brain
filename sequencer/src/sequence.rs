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

#[derive(Debug, Clone, Hash, Eq)]
pub struct Step(i32);

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug, Clone)]
pub struct Sequence {
    triggers: HashMap<Step, HashSet<Trigger>>,
    number_of_steps_in_sequence: i32,
}

impl Sequence {
    pub fn empty() -> Sequence {
        let mut triggers = HashMap::new();

        for n in 1..=16 {
            triggers.insert(Step(n), HashSet::new());
        }

        Sequence {
            triggers: triggers,
            number_of_steps_in_sequence: 16,
        }
    }

    pub fn triggers_for_tick(&self, tick: i32) -> Vec<Trigger> {
        let ticks_per_step = 6;
        let sequence_length_in_ticks = self.number_of_steps_in_sequence * ticks_per_step;
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
        let sequence_length_in_ticks = self.number_of_steps_in_sequence * ticks_per_step;
        let offset_into_sequence = tick % sequence_length_in_ticks;

        Step((offset_into_sequence / ticks_per_step) + 1)
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
