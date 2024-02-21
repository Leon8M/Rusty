use std::io;

fn is_palindrome(word: &str) -> bool {
    let lowercase_word = word.to_lowercase();

    lowercase_word.chars().eq(lowercase_word.chars().rev())
}

fn main() {
    // Prompt the user to enter a word
    println!("Enter a word:");

    // Read the input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim leading and trailing whitespaces
    let word = input.trim();

    // Check if the entered word is a palindrome
    if is_palindrome(word) {
        println!("{} is a palindrome!", word);
    } else {
        println!("{} is not a palindrome.", word);
    }
}
