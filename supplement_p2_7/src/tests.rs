use number_converter::*;

#[test]
fn test_i32_to_f32_and_back() {
    let i: i32 = 1234567890;
    let f = i32_to_f32(i);
    let i2 = f32_to_i32(f);
    assert_eq!(f32_to_i32(i32_to_f32(i)), i2);
}
#[test]
fn test_u32_to_f32_and_back() {
    let u: u32 = 0x40400000;
    let f = u32_to_f32(u);
    let u2 = f32_to_u32(f);
    assert_eq!(u, u2);
}
#[test]
fn test_i64_to_f64_and_back() {
    let i: i64 = 922337203685477580;
    let f = i64_to_f64(i);
    let i2 = f64_to_i64(f);
    assert_eq!(i, i2);
}
#[test]
fn test_u64_to_f64_and_back() {
    let u: u64 = 4621819117588971520;
    let f = u64_to_f64(u);
    let u2 = f64_to_u64(f);
    assert_eq!(u, u2);
}

