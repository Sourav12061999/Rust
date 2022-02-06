// structs are used to create custom data types

struct Color {
    red: i32,
    blue: u8,
    green: u8,
}
// Tuple struct
struct Color2(u8, u8, u8);

// similar to methods and constructor function in javascript
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct Person
    fn newfunction(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    // Adding method to it
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    // Muting struct value with function
    fn change_lastname(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Make first and last name tuple
    fn tuple_name(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}
pub fn run() {
    let mut c = Color {
        red: 255,
        blue: 0,
        green: 0,
    };
    println!("Structs red:-{} blue:-{} green:-{}", c.red, c.blue, c.green);
    c.blue = 5; // Changing value of a struct. but for that let has to be mutable
    println!("Structs red:-{} blue:-{} green:-{}", c.red, c.blue, c.green);

    let mut c2 = Color2(5, 6, 7);
    c2.0 = 9;
    println!("Tuple Structs 0:-{} 1:-{} 2:-{}", c2.0, c2.1, c2.2);

    let mut person = Person::newfunction("Sourav", "Das");
    println!("Full Name:- {}", person.full_name());
    person.change_lastname("Dey"); // This is mutating the lastname through a function
    println!("Full Name:- {}", person.full_name());
    println!("Tuple Full Name:- {:?}", person.tuple_name());
}
