/*
Primitive Types:
Integars: u8, i8, i16, u32, i32, u64, i64, u128, i128 (signed or unsigned) (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed lanuage, which means that it must know the types of all variables at the compile time, 
// however, the compiler can usually infer what type we want to use based on the value and how we use it


pub fn run() {
    //Default is "i32"
    let _x = 1;

    //Default is "f64"
    let _y = 2.5;

    //Add explicit type
    let _z: i64 = 4545454545454;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active: bool = true;

    //Get Boolean from expression
    let _is_greater: bool = 10 < 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}",(_x, _y, _z, is_active,a1,face));
}