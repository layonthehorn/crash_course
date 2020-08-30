pub fn run(){
    // reference pointers point to resources in memory

    // primitive array
    // can be copied directly
    let arr1 = [1,2,3];
    let arr2 = arr1;

    // non primitives will need to pointed with references, or else the
    // original will not hold the values they used to.

    // vector
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("{:?}", (&vec1, vec2 ));
}