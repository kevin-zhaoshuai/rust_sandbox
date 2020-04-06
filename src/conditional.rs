// Conditionals - Used to check the condition of sth and act

pub fn run() {
    let age: u8 = 22;
    let check_id: bool = false;
    let knows_person_of_age = true;

    // If/Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: Waht would you like to drink?");
    } else if age < 21 && check_id {
        println!("Sorry you have to leave");
    } else {
        println!("I will needto see your ID");
    }

    // Shorthand If
    let is_of_age = if age >= 21 { true } else { false };
}