// stores code for reuse

pub fn run(){
    greeting("Hello", "Darren");
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // closure function
    let n3 = 10;
    // allowing using variables outside the function
    let add_nums = |n1:i32, n2:i32| n1 + n2 + n3;
    println!("Closure sum: {}", add_nums(3,2));
}

fn greeting(greet: &str, name: &str){
    println!("{} {}, nice to meet you.", greet, name)
}

// takes two numbers and returns a number
fn add(n1: i32, n2:i32) -> i32{
    // no semicolon here means this line is the return
    n1 + n2
}