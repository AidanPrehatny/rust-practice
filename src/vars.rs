pub fn run() {
    let name = "Brad";
    // allow variable to be mutated
    let mut age = 40;
    println!("My name is {} and I am {}", name, age);
    age = 55;
    println!("My name is {} and I am {}", name, age);
    // define constant, have to add type
    const ID: i32 = 001;
    println!("ID: {}", ID);
}