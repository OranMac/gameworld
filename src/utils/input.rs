use std::io::{self, Write};

/// Reads a line from the console and trims whitespace.
pub fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // flust ensures prompt appears before input

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

/// Reads input and ensures it matches one of the allowed options.
/// Keeps retrying until valid.
pub fn read_choice(prompt: &str, options: &[&str]) -> String {
    loop {
        let input = read_line(prompt);

        if options.contains(&input.as_str()) {
            return input;
        }

        println!("Invalid choice, please try again.");
    }
}
