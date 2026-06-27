use rand::Rng;
use std::io;

fn password_strength(len: usize) -> &'static str {
    match len {
        0..=6 => "Weak",
        7..=11 => "Medium",
        12..=17 => "Strong",
        _ => "Very Strong",
    }
}

fn read_usize(prompt: &str) -> usize {
    loop {
        println!("{}", prompt);
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse() {
            Ok(n) => return n,
            Err(_) => println!("Invalid input. Enter a number:"),
        }
    }
}

fn main() {
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&'()*+,-./:;<=>?@[]^_`{|}~"
            .chars()
            .collect();
    loop {
        let length = read_usize("Enter the length of the password: ");
        if length == 0 {
            println!("Length must be at least 1.");
            continue;
        }
        let count = read_usize("How many passwords do you want? ");

        let mut rng = rand::rngs::OsRng;
        let passwords: Vec<String> = (0..count)
            .map(|_| {
                (0..length)
                    .map(|_| charset[rng.gen_range(0..charset.len())])
                    .collect()
            })
            .collect();

        for (i, password) in passwords.iter().enumerate() {
            println!("Generated password {}: {}", i + 1, password);
        }
        println!("Password strength: {}", password_strength(length));

        println!("Do you want to copy a password? (y/n): ");
        let mut copy_choice = String::new();
        io::stdin()
            .read_line(&mut copy_choice)
            .expect("Failed to read line");

        if copy_choice.trim() == "y" {
            println!("Copy all or one? (all/one): ");
            let mut all_or_one = String::new();
            io::stdin()
                .read_line(&mut all_or_one)
                .expect("Failed to read line");

           let mut clipboard = arboard::Clipboard::new().expect("Failed to access clipboard");

            if all_or_one.trim() == "all" {
                let all = passwords.join("\n");
                clipboard.set_text(&all).expect("Failed to copy");
                println!("All passwords copied to clipboard!");
                println!(
                    "WARNING: do NOT paste into a password field or browser address bar. All passwords will appear as one block of text."
                );
            } else if all_or_one.trim() == "one" {
                println!("Which one? (1-{}): ", count);
                let mut copy_input = String::new();
                io::stdin()
                    .read_line(&mut copy_input)
                    .expect("Failed to read line");
                match copy_input.trim().parse::<usize>() {
                    Ok(i) if i >= 1 && i <= passwords.len() => {
                        clipboard
                            .set_text(&passwords[i - 1])
                            .expect("Failed to copy");
                        println!("Password {} copied to clipboard!", i);
                    }
                    _ => println!("Invalid number."),
                }
            }
        }

        println!("Do you want to generate more passwords? (y/n): ");
        let mut continue_choice = String::new();
        io::stdin()
            .read_line(&mut continue_choice)
            .expect("Failed to read line");
        if continue_choice.trim() != "y" {
            println!("Goodbye!");
            break;
        }
    }
}
