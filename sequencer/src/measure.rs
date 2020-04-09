#[derive(Clone, Debug)]
pub struct Measure(pub i32, pub i32);

impl Measure {
    pub fn to_ms(&self, bpm: f32) -> u64 {
        let ms_per_beat = (60. / bpm) * 1000.;
        let length_of_measure_in_beats = 4. / self.1 as f32;

        (length_of_measure_in_beats * (self.0 as f32) * ms_per_beat).ceil() as u64
    }

    pub fn to_float(&self) -> f32 {
        self.0 as f32 / self.1 as f32
    }

    pub fn reduce_to_one_bar(&self) -> Measure {
        let range = 1..(self.1 + 1);
        let a = range.cycle().take(self.0 as usize).last().unwrap();
        Measure(a, self.1)
    }
}

impl PartialEq for Measure {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[test]
fn test_to_ms() {
    let measure = Measure(1, 16);
    let bpm = 60.0;

    assert_eq!(250, measure.to_ms(bpm));

    let measure = Measure(1, 4);
    let bpm = 120.0;

    assert_eq!(500, measure.to_ms(bpm));

    let measure = Measure(2, 4);
    let bpm = 120.0;

    assert_eq!(1000, measure.to_ms(bpm));
}

#[test]
fn test_equality() {
    assert!(Measure(1, 4) == Measure(1, 4))
}

#[test]
fn test_to_float() {
    assert_eq!(Measure(1, 4).to_float(), 0.25);
}

#[test]
fn test_reduce_to_one_bar() {
    assert_eq!(Measure(5, 4).reduce_to_one_bar(), Measure(1, 4));
    assert_eq!(Measure(17, 16).reduce_to_one_bar(), Measure(1, 16));
}
