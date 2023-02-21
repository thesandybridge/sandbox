use std::fs;

fn main() {
    let file = fs::read_to_string("../stuff.txt")
        .expect("Something went wrong");

    let lines = file
        .lines()
        .collect::<Vec<&str>>();

    println!("{:?}", lines);
}
