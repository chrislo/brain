use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Trigger {
    note_number: i32,
    offset: i32,
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
    triggers: HashMap<Step, Trigger>,
    number_of_steps_in_sequence: i32,
}

impl Sequence {
    pub fn empty() -> Sequence {
        Sequence {
            triggers: HashMap::new(),
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
            .into_iter()
            .filter(|t| t.offset == offset_into_step)
            .cloned()
            .collect()
    }

    pub fn trigger_note_number_at_step(&self, note_number: i32, step: Step) -> Sequence {
        let trigger = Trigger {
            note_number: note_number,
            offset: 0,
        };
        let mut triggers = self.triggers.clone();
        triggers.insert(step, trigger);

        Sequence {
            triggers: triggers,
            ..self.clone()
        }
    }
}

#[test]
fn test_adding_trigger_to_sequence() {
    let sequence = Sequence::empty().trigger_note_number_at_step(1, Step(1));

    for n in 0..96 {
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
