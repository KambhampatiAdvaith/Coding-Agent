/* Main Code (Rust entry point) */

use std::io::{self, Write};

fn main() {
    println!("🤖 Welcome to Coding-Agent CLI!");

    loop {
        print!("> Enter your prompt (or type 'exit'): ");
        io::stdout().flush().unwrap();

        let mut prompt = String::new();
        io::stdin()
            .read_line(&mut prompt)
            .expect("Failed to read input");

        let prompt = prompt.trim();

        if prompt.eq_ignore_ascii_case("exit") {
            println!("👋 Exiting Coding-Agent. Happy coding!");
            break;
        }

        let response = process_prompt(prompt);

        println!("🧠 AI Response:\n{}", response);
    }
}

fn process_prompt(input: &str) -> String {
    format!("Generated code for: '{}'\n// TODO: Integrate AI code generation logic here.", input)
}
