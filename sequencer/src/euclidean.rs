pub fn euclidean_pattern(onsets: i32, pulses: i32) -> Vec<i32> {
    let slope = onsets as f32 / pulses as f32;
    let mut previous = 1;
    let mut result = vec![0; pulses as usize];

    if onsets == 0 {
        return result;
    }

    for i in 0..pulses {
        let current = (i as f32 * slope).floor() as i32;
        if current != previous {
            result[i as usize] = 1;
        }
        previous = current;
    }
    result
}

#[test]
fn test_euclidean_pattern() {
    let pattern = euclidean_pattern(4, 16);
    assert_eq!(
        pattern,
        vec!(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0)
    );

    let pattern = euclidean_pattern(5, 12);
    assert_eq!(pattern, vec!(1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0));

    let pattern = euclidean_pattern(0, 4);
    assert_eq!(pattern, vec!(0, 0, 0, 0));

    let pattern = euclidean_pattern(4, 4);
    assert_eq!(pattern, vec!(1, 1, 1, 1));
}
