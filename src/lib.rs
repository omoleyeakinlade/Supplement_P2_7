pub fn i32_to_f32(n: i32) -> f32 {
    f32::from_bits(n as u32)
   }

   pub fn f32_to_i32(f: f32) -> i32 {
    f.to_bits() as i32
}

pub fn u32_to_f32(n: u32) -> f32 {
    f32::from_bits(n)
}

pub fn f32_to_u32(f: f32) -> u32 {
    f.to_bits()
} 
