fn main() {
    println!("Welcome to the Number Conversion App!");

    let int_val: i32 = 1065353216;
    let float_val = number_converter::i32_to_f32(int_val);
    println!("i32 {} interpreted as f32: {}", int_val, float_val);
}