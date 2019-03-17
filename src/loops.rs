pub fn run() {
    let mut count = 0;
    let mut fizzbuzzer = 0;
    let mut fizzbuzzerRange = 0;

    // Infinite Loop
    // loop {
    //     count +=1;
    //     println!("Number is: {}", count);
    
    //     if count == 20 {
    //         break;
    //     }
    // }
    

    // While Loop (Fizz Buzz)
    while fizzbuzzer <= 100 {
        if fizzbuzzer % 15 == 0 {
            println!("fizzbuzz");
        } else if fizzbuzzer % 3 == 0 {
            println!("fizz");
        } else if fizzbuzzer % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", fizzbuzzer);
        }  
        // Inv
        fizzbuzzer += 1;
    }

    // For Range
    for x in 0..100 {
        if fizzbuzzerRange % 15 == 0 {
            println!("fizzbuzz");
        } else if fizzbuzzerRange % 3 == 0 {
            println!("fizz");
        } else if fizzbuzzerRange % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", fizzbuzzerRange);
        }  
        // Inv
        fizzbuzzerRange += 1;
    } 
    

}