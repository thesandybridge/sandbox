use clap::Parser;

/// Simple CLI to encode a message using a skip cipher with wrap-around
#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Encodes messages using a skip cipher with wrap-around", long_about = None)]
struct Args {
    /// The message to encode
    #[arg(short, long)]
    message: String,

    /// The number of characters to skip
    #[arg(short, long)]
    skip: usize,
}

fn skip_cipher(input: &str, skip: usize) -> String {
    let mut result = String::new();
    let chars: Vec<char> = input.chars().collect();
    let length = chars.len();
    let mut i = 0;

    while i < length {
        result.push(chars[i]);
        i = (i + skip) % length; // Wrap around using modulo
    }

    result
}

fn main() {
    let args = Args::parse();

    let encoded_message = skip_cipher(&args.message, args.skip);
    println!("Encoded message: {}", encoded_message);
}
