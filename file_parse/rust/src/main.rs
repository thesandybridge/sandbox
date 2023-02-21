use std::fs;
use std::env;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let path = args[1].parse::<String>().unwrap();

    let file = fs::read_to_string(path)
        .expect("Something went wrong");

    let lines = file
        .lines()
        .collect::<Vec<&str>>();

    let end = start.elapsed();
    println!("Exec Time: {:?}", end);
    println!("Array Length: {:?}", lines.len());
}
