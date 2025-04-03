#[test]
fn test_i32_to_f32_and_back() {
    let i: i32 = 1234567890;
    let f = i32_to_f32(i);
    let i2 = f32_to_i32(f);
    assert_eq!(f32_to_i32(i32_to_f32(i)), i2);
}
