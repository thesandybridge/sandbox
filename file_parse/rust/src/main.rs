use std::fs;
use std::env;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let file = args[1].parse::<String>().unwrap();

    match args.len() {
        2 => {
            let file = fs::read_to_string(file)
                .expect("Something went wrong");

            let lines = file
                .lines()
                .collect::<Vec<&str>>();

            println!("{:?}", lines);

        },
        _ => println!("Need at least 1 argument for file path")
    }
    let end = start.elapsed();
    println!("Exec Time: {:?}", end);
}
