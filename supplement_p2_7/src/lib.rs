/// # Description
/// Reinterprets a 32-bit signed integer as a 32-bit floating-point number
///
/// # Parameters
/// - n: A 32-bit signed integer to reinterpret.
///
/// # Returns
/// - A f32 with the same underlying 32-bit binary pattern as n.
pub fn i32_to_f32(n: i32) -> f32 {
    f32::from_bits(n as u32)
   }
/// # Description.
/// Reinterprets a 32-bit floating-point number as a 32-bit signed integer 
/// # Parameters.
/// - f: A 32-bit float to reinterpret.
///
/// # Returns
/// - An i32 with the same underlying 32-bit binary pattern as f.
   pub fn f32_to_i32(f: f32) -> i32 {
    f.to_bits() as i32
}
/// # Description
/// Reinterprets a 32-bit unsigned integer  as a 32-bit floating-point number 
/// using raw byte-level representation.
///
/// # Parameters
/// - n: A 32-bit unsigned integer to reinterpret.
///
/// # Returns
/// - A f32 with the same underlying 32-bit binary pattern as n.

pub fn u32_to_f32(n: u32) -> f32 {
    f32::from_bits(n)
}
/// # Description
/// Reinterprets a 32-bit floating-point number  as a 32-bit unsigned integer 
/// using raw byte-level representation.
///
/// # Parameters
/// - f: A 32-bit float to reinterpret.
///
/// # Returns
/// - A u32 with the same underlying 32-bit binary pattern as f.

pub fn f32_to_u32(f: f32) -> u32 {
    f.to_bits()
} 
///  # Description
/// Reinterprets a 64-bit signed integer as a 64-bit floating-point number 
/// using raw byte-level representation.
///
/// # Parameters
/// - n: A 64-bit signed integer to reinterpret.
///
/// # Returns
/// - A f64 with the same underlying 64-bit binary pattern as n.
pub fn i64_to_f64(n: i64) -> f64 {
    f64::from_bits(n as u64)
} 
/// # Description 
/// Reinterprets a 64-bit floating-point number as a 64-bit signed integer 
/// using raw byte-level representation.
///
/// # Parameters
/// - f: A 64-bit float to reinterpret.
///
/// # Returns
/// - An i64 with the same underlying 64-bit binary pattern as f.
pub fn f64_to_i64(f: f64) -> i64 {
    f.to_bits() as i64
} 
/// # Description 
/// Reinterprets a 64-bit unsigned integer  as a 64-bit floating-point number
///
/// # Parameters
/// - n: A 64-bit unsigned integer to reinterpret.
///
/// # Returns
/// - A f64 with the same underlying 64-bit binary pattern as n.

pub fn u64_to_f64(n: u64) -> f64 {
    f64::from_bits(n)
}
/// # Description 
/// Reinterprets a 64-bit floating-point number  as a 64-bit unsigned integer 
/// using raw byte-level representation.
///
/// # Parameters
/// - f: A 64-bit float to reinterpret.
///
/// # Returns
/// - A u64 with the same underlying 64-bit binary pattern as f.
pub fn f64_to_u64(f: f64) -> u64 {
    f.to_bits()
}


