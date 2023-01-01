use std::fs;

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; 
    let file = fs::read_to_string("array.txt")
        .expect("Something went wrong reading the file");
    let mut values = file
        .split(",")
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<i32>>();
    vec.append(&mut values);
    println!("{:?}", vec)
}
