// tuples group together values
// max is 12 elements

pub fn run(){
   let person: (&str, &str, i8) = ("Thomas", "Oregon", 23);

   println!("{} lives in {} and is {}", person.0, person.1, person.2)
}