/*
ints: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 number of bits they take in memory
floats: f32 f64
boolean: (bool) true, false
characters: char '1', 'f'
tuples: ()
arrays: []

Rust is a statically typed language and requires set data types for each variable.
However, it can infer some types.
*/

pub fn run(){
    // rust inferring the type itself
    let x = 1;
    let y = 2.5;
    // us stating the type if needed
    let z: i64 = 2000000000;

    // this gives the max value acceptable in the
    // data type selected.
    println!("Max i32 {}",std::i32::MAX);
    println!("Max i64 {}",std::i64::MAX);

    // booleans
    // unlike python is not capitalized
    let is_active = true;

    // get boolean from expression
    let is_greater = 10 > 5;
    println!("{:?}", (x,y,z,is_active,is_greater));

    // characters
    // can only be one character and be within single quotes
    // can accept any unicode character, including emojis
    let a1 = 'a';
}