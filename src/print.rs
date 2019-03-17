pub fn run() {
    // print from console
    println!("Gucci Prada");

    // basic format
    println!("{} is {}", "gucci", "cool");

    // positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Tom", "Hollywood", "program"
    );

    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    println!("{:?}", (12, true, "hello"));

}
