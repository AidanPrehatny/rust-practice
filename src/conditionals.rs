pub fn run() {
    let age: u8 = 205;
    let check_id: bool = true;
    let knows_person_of_age: bool = true;

    // if/else
    if age >= 21 && age < 100 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Go get a fake ID");
    } else if age > 200 {
        println!("Bartender: Woah your a vampire thats dope.");
    } else {
        println!("Bartender: Ill need to see your ID");
    }

    // Shorthand If
    let is_of_age = if age >= 21 { true } else { false };
    println!("is of age: {}", is_of_age);
    
}