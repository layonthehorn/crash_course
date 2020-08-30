// used to create custom data types

// holds primitives and functions
// traditional struct type
struct Color{
    red: u8,
    green: u8,
    blue: u8
}
// tuple struct
struct ColorTup(u8, u8, u8);

// struct with functions
struct Person {
    first_name: String,
    last_name: String
}

// Allows creating functions attached to the struct person.
// this creates the struct person and takes in the names as
// arguments.
impl Person {
    fn new(first: &str, last: &str ) ->Person{
       Person {
           first_name: first.to_string(),
           last_name: last.to_string()
       }
    }

    // getting the full name
    fn full_name(&self) -> String{
        // Remember! no semicolon to return this result
        // format! is a macro like println! but does not print the
        // formatted string, only returns it.
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    // use mut self to allow changing the internal data
    fn set_last_name(&mut self, last: &str){
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }
}

pub fn run(){
    let mut c = Color{
        red: 255,
        green: 0,
        blue: 0
    };

    println!("Color: {} {} {}",c.red, c.green, c.blue);
    // allows direct access to contents
    c.red = 200;
    println!("Color: {} {} {}",c.red, c.green, c.blue);

    let mut c_t = ColorTup(255, 0, 0);

    println!("Color: {} {} {}",c_t.0, c_t.1, c_t.2);
    c_t.0 = 200;
    println!("Color: {} {} {}",c_t.0, c_t.1, c_t.2);

    // Person testing
    let mut p = Person::new("Darren", "Capper");
    println!("Person: {} {}",p.first_name, p.last_name);
    println!("Person Full Name: {}",p.full_name());
    println!("To Tuple: {:?}",p.to_tuple());
}