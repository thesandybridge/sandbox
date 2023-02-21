use std::fs;
use std::env;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let path = args[1].parse::<String>().unwrap();

    let file = fs::read_to_string(path)
        .expect("Something went wrong");

    let mut lines = file
        .lines()
        .map(|s| s
             .chars()
             .filter_map(|c| c.is_digit(10).then(|| c))
             .collect::<String>()
            )
        .collect::<Vec<String>>();
    
    lines.retain(|s| !s.is_empty());

    let filtered_lines = lines
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let end = start.elapsed();
    println!("Exec Time: {:?}", end);
    println!("Array Length: {:?}", filtered_lines.len());
}
