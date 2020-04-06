// Struct - Used to create custom data types

// // Traditional struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8
// }

// // Tuple struct
// struct ColorNew(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    //Get Full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}


pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0
    // };
    // c.red = 200;
    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    // let mut d = ColorNew(255,0,0);
    // d.2 = 100;
    // println!("Color: {} {} {}", d.0, d.1, d.2);
    
    let mut p = Person::new("John", "Doe");
    println!("Person {}", p.full_name());
    p.set_last_name("Williams");
    println!("Person {}", p.full_name());
    println!("Person Tuple {:?}", p.to_tuple());
}
