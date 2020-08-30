pub fn run(){

    // allows getting arguments off the command line
    let args: Vec<String> = std::env::args().collect();
    // clones the value in the args vector
    let command = args[1].clone();
    let status = "100 %";
    println!("Command: {}", command);

    if command == "hello" {
        println!("Hi, how are you?");
    } else if command == "status"{
        println!("Status is, {}", status);
    } else{
        println!("That is not a valid command.");
    }
}