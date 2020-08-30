// checks condition, if/else

pub fn run(){
    let age:u8 = 34;
    let check_id = false;
    let knows_age = true;

    // if else statement
    // && is and
    // || is or
    if age >= 21 && check_id || knows_age{
        println!("You can drink.");
    } else if age < 21 && check_id{
        println!("You can't drink.");
    } else {
        println!("Can I see your ID?");
    }

    // short hand if and else statement
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of age: {}", is_of_age)

}