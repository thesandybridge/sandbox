use std::fs;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args[1].parse::<String>().unwrap();

    let file = fs::read_to_string(path)
        .expect("Something went wrong");

    let start = Instant::now();
    let lines = file
        .lines()
        .map(|s| s
             .chars()
             .filter_map(|c| c.is_alphabetic().then(|| c))
             .collect::<String>()
            )
        .collect::<Vec<String>>();

    let end = start.elapsed();
    println!("Exec Time: {:?}", end);
    println!("Array Length: {:?}", lines.len());
}
