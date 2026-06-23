use std::io;
use rand::Rng;

fn password_strength(len: usize) -> &'static str {
    match len {
        0..=6 => "Weak",
        7..=11 => "Medium",
        12..=17 => "Strong",
        _ => "Very Strong",
    }
}

fn main() {
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&'()*+,-./:;<=>?@[]^_`{|}~"
        .chars()
        .collect();

    println!("Enter the length of the password: ");

    let mut input = String::new(); 
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let length: usize = input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let mut rng: rand::rngs::OsRng = rand::rngs::OsRng;
    let password: String = (0..length)
        .map(|_| charset[rng.gen_range(0..charset.len())])
        .collect();

    println!("Generated password: {}", password);

    let mut clipboard: arboard::Clipboard = arboard::Clipboard::new().expect("Failed to access clipboard");
    clipboard.set_text(&password).expect("Failed to copy");
    println!("Password copied to clipboard!");
    println!("Password strength: {}", password_strength(length));
}
