use std::collections::HashMap;
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    println!("Enter the message and keyword separated by a comma:");
    let input = iterator.next().unwrap().unwrap();
    let parts: Vec<&str> = input.split(',').collect();
    if parts.len() != 2 {
        writeln!(
            io::stderr(),
            "Please provide the message and the keyword separated by a comma."
        )
        .unwrap();
        return;
    }

    let message = parts[0].replace(" ", "").to_uppercase();
    let keyword = parts[1].trim();

    let encryption = encrypt(&message, keyword);
    println!("Encrypted message: {}", encryption);
}

fn encrypt(message: &str, keyword: &str) -> String {
    let keyword_len = keyword.len();
    let mut sorted_keyword = keyword.chars().enumerate().collect::<Vec<(usize, char)>>();
    sorted_keyword.sort_by(|a, b| a.1.cmp(&b.1));

    let mut order_map = HashMap::new();
    for (i, (original_pos, _)) in sorted_keyword.into_iter().enumerate() {
        order_map.insert(original_pos, i);
    }

    let mut cols = vec![String::new(); keyword_len];
    for (i, c) in message.chars().enumerate() {
        let col = i % keyword_len;
        cols[col].push(c);
    }

    let mut result = String::new();
    for i in 0..keyword_len {
        if let Some(&col) = order_map.get(&i) {
            result.push_str(&cols[col]);
        }
    }

    result
}
