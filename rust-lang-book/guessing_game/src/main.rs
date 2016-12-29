use std::io;


fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();  // Test comment.

    io::stdin().read_line(&mut guess)  // Apparently newlines don't matter.
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

}
