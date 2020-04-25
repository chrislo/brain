use crate::event::Event;
use crate::measure::Measure;

#[derive(Debug, Clone)]
pub struct Track {
    events: Vec<Event>,
}

impl Track {
    pub fn empty() -> Track {
        Track { events: vec![] }
    }

    pub fn events_between(&self, start: Measure, end: Measure) -> Vec<Event> {
        let start_float = start.reduce_to_one_bar().to_float();
        let end_float = end.reduce_to_one_bar().to_float();

        self.events
            .clone()
            .into_iter()
            .filter(|e| e.start.to_float() > start_float && e.start.to_float() <= end_float)
            .collect::<Vec<Event>>()
    }

    pub fn toggle_step(&self, measure: Measure) -> Track {
        let event = Event { start: measure };

        if self.missing(event) {
            self.add_event(event)
        } else {
            self.remove_event(event)
        }
    }

    pub fn active_steps(&self) -> Vec<Measure> {
        self.events
            .clone()
            .into_iter()
            .map(|e| e.start)
            .collect::<Vec<Measure>>()
    }

    fn add_event(&self, event: Event) -> Track {
        let mut events = self.events.clone();
        events.push(event);
        Track { events: events }
    }

    fn remove_event(&self, event: Event) -> Track {
        let mut events = self.events.clone();
        events.retain(|e| *e != event);
        Track { events: events }
    }

    fn missing(&self, event: Event) -> bool {
        !self.events.contains(&event)
    }
}

#[test]
fn test_events_between() {
    let track = Track::empty().toggle_step(Measure(2, 16));

    let events = track.events_between(Measure(1, 16), Measure(3, 16));
    assert_eq!(Measure(2, 16), events[0].start);

    let events = track.events_between(Measure(3, 16), Measure(4, 16));
    assert!(events.is_empty());

    let events = track.events_between(Measure(17, 16), Measure(19, 16));
    assert_eq!(Measure(2, 16), events[0].start);

    let track = Track::empty().toggle_step(Measure(1, 16));
    let events = track.events_between(Measure(1, 32), Measure(2, 32));
    assert_eq!(Measure(1, 16), events[0].start);
}

#[test]
fn test_active_steps() {
    assert_eq!(0, Track::empty().active_steps().len());

    let active_steps = Track::empty()
        .toggle_step(Measure(1, 16))
        .toggle_step(Measure(16, 16))
        .active_steps();

    assert_eq!(active_steps, vec![Measure(1, 16), Measure(16, 16)]);
}

#[test]
fn test_toggle_step() {
    let track = Track::empty();

    let processed_track = track.toggle_step(Measure(2, 16));
    assert_eq!(
        1,
        processed_track
            .events_between(Measure(1, 16), Measure(16, 16))
            .len()
    );

    let processed_track = processed_track.toggle_step(Measure(2, 16));
    assert_eq!(
        0,
        processed_track
            .events_between(Measure(1, 16), Measure(16, 16))
            .len()
    );
}
