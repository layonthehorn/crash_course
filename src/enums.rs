// types with a few definite values

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement){
    // preform action based off of input
    match m {
        Movement::Up => println!("Moved up"),
        Movement::Down => println!("Moved down"),
        Movement::Left => println!("Moved left"),
        Movement::Right => println!("Moved right")

    }
}

pub fn run(){

    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Down;
    let avatar4 = Movement::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);


}