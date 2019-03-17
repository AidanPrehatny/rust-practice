enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    // Perform action depending on info
    match m {
        Movement::Up => println!("Avatrar moving up")
        Movement::Down => println!("Avatrar moving down")
        Movement::Left => println!("Avatrar moving left")
        Movement::Right => println!("Avatrar moving right")
    } // match is like a switch
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}