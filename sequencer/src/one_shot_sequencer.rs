use crate::event::Event;

#[derive(Debug, Clone)]
pub struct OneShotSequencer {
    one_shots: Vec<Step>,
}

#[derive(Clone, Copy, Debug)]
struct Step {
    tick: i32,
    note_number: i32,
}

impl OneShotSequencer {
    pub fn empty() -> OneShotSequencer {
        OneShotSequencer { one_shots: vec![] }
    }

    pub fn events_for_tick(&self, tick: i32) -> Vec<Event> {
        self.one_shots
            .iter()
            .filter(|s| s.tick == tick)
            .map(|s| Event {
                note_number: s.note_number,
            })
            .collect()
    }

    pub fn add_one_shot(&self, note_number: i32, tick: i32) -> OneShotSequencer {
        let mut one_shots: Vec<Step> = self
            .one_shots
            .clone()
            .into_iter()
            .filter(|s| s.tick == tick)
            .collect();

        let step = Step {
            tick: tick,
            note_number: note_number,
        };

        one_shots.push(step);

        OneShotSequencer {
            one_shots: one_shots,
        }
    }
}

#[test]
fn test_add_one_shot() {
    let note_number = 37;
    let tick = 6;
    let sequencer = OneShotSequencer::empty().add_one_shot(note_number, tick);

    let events = sequencer.events_for_tick(tick);
    assert_eq!(note_number, events[0].note_number);
}

#[test]
fn test_add_one_shot_clears_old_one_shots() {
    let note_number = 37;
    let sequencer = OneShotSequencer::empty()
        .add_one_shot(note_number, 6)
        .add_one_shot(note_number, 7)
        .add_one_shot(note_number, 7);

    assert_eq!(0, sequencer.events_for_tick(6).len());
    assert_eq!(2, sequencer.events_for_tick(7).len());
}
