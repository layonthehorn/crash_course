// fixed length element where all the numbers are the same data types

pub fn run(){
    // array of length 5 and data type integer 32 bit
    // [1,2,3,4] would not compile because it needs to contain 5 elements
    let mut numbers: [i32; 5]= [1,2,3,4,5];

    // reassign a value
    numbers[2] = 20;

    // printing whole array must be done with debug print
    println!("{:?}", numbers);

    // printing a single element
    println!("Single Value: {}", numbers[0]);

    // getting length of array
    println!("Length: {}", numbers.len());

    // Arrays are allocated on the stack
    println!("Array occupies {} bytes.", std::mem::size_of_val(&numbers));

    // get slice from index to index
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice)
}