use supplement_p2_7::i32_to_f32;
///
///
fn main() {
    println!("Welcome to the Number Conversion App!");

    let int_val: i32 = 1065353216;
    let float_val = i32_to_f32(int_val);
    println!("i32 {} interpreted as f32: {}", int_val, float_val);
}