// growable lists of elements
// much is the same as arrays but allow you to change size after creation

pub fn run(){
    // defining a vector. Vec<i32> must be capitalized
    let mut numbers: Vec<i32>= vec![1,2,3,4,5];

    // reassign a value
    numbers[2] = 20;

    // Adding to vector
    numbers.push(5);
    numbers.push(6);

    // popping off last value
    numbers.pop();

    // printing whole array must be done with debug print
    println!("{:?}", numbers);

    // printing a single element
    println!("Single Value: {}", numbers[0]);

    // getting length of array
    println!("Length: {}", numbers.len());

    // Arrays are allocated on the stack
    println!("Vector occupies {} bytes.", std::mem::size_of_val(&numbers));

    // get slice from index to index
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // loop through vector values
    for x in numbers.iter(){
      println!("Number: {}", x)
    };

    // loop through vector values
    // and mutates them
    for x in numbers.iter_mut(){
        *x *= 2;
    };
    println!("Numbers in Vector: {:?}", numbers);
}
