/*
types of strings,
immutable fixed length type, str
growable heap allocated data structure, string

 */

use std::process::id;

pub fn run(){
    // this will allow us to change the string later
    let mut hello = String::from("Hello");
    // getting length
    println!("Length: {}", hello.len());

    // pushing a character onto the end
    hello.push(' ');
    // pushing a string to the end of a string
    hello.push_str("Darren.");
    println!("{}", hello);

    // capacity of string in bytes
    println!("Capacity: {}", hello.capacity());

    // checking if string is empty
    println!("Is empty: {}", hello.is_empty());

    // checking for substring
    println!("Contains Darren: {}", hello.contains("Darren"));

    // replacing substring
    println!("Replace: {}", hello.replace("Darren", "pretty lion man"));

    // loop through string by white space
    // splits on whitespace and loop through the return
    for word in hello.split_whitespace(){
        println!("{}", word);
    };

    // create new string with specified capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing
    // if either fail the test the program crashes
    // checks if the string length is exactly two
    assert_eq!(2, s.len());
    // checks if capacity is 10
    assert_eq!(10, s.capacity());
}