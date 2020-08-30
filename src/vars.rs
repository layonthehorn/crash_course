pub fn run(){
    // immutable variable
    let name = "Thomas";
    // mutable variable
    let mut age = 23;
    // constants must be defined with a data type.
    // in this case it is a integer 32 bit
    const ID: i32 = 001;

    println!("My name is {} and I am {}", name, age);
    age = 24;
    println!("My name is {} and I am {}", name, age);
    println!("ID: {}", ID);

    // assign multiple variables
    let (my_name, my_age) = ("Darren", 27);
    println!("Big lion's name {} and age, {}", my_name, my_age)
}