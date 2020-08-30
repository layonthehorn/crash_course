pub fn run(){
    // print to console
    println!("Hello from the print rs file.");
    // basic formatting
    // a string you can include as many things as you wish
    println!("The number {}.", 1);

    // positional
    println!("{0} is my best {1}, I love {0}", "Darren", "Friend");

    // named arguments
    println!("{name} likes to play {activity}", name = "Thomas", activity = "Stardew Valley");

    // place holder traits
    // these will convert numbers to the base values shown here
    println!("Binary: {:b}\nHex: {:x}\nOctal: {:o}",10,10,10);

    // debug trait
    println!("{:?}",(12, true, "Hello"));

    // basic math
    println!("10 + 10 = {}", 10+10)
}