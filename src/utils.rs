pub fn checked_sum_u32(numbers: &[u32]) -> Option<u32> {
    let mut result: Option<u32> = Option::None;
    numbers.iter().for_each(|v| result = result.unwrap_or(0).checked_add(*v));
    result
}

#[test]
fn test() {
    assert_eq!(checked_sum_u32(&[32, 34, 5351100]).unwrap_or(0), 5351166);
    assert_eq!(checked_sum_u32(&[3200344322, 34, 3351100341]), Option::None);
}
