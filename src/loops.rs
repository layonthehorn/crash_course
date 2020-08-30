pub fn run(){
    let mut count = 0;
    // simple loop
    loop{
        // adds one to count
        count += 1;
        println!("Number, {}", count);

        // if count equals 20 we stop looping
        if count == 20{
            break;
        }
    }

    // while loop FizzBuzz
    count = 0;
    while count <= 100{
        if count % 15 == 0{
            println!("Fizzbuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0{
            println!("Buzz");
        } else {
            println!("{}", count);
        }
        count += 1;

    }

    // for range loop
    // prints numbers 0 to 100
    for x in 0..100{

        if x % 15 == 0{
            println!("Fizzbuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0{
            println!("Buzz");
        } else {
            println!("{}", x);
    }
}