use std::fs;
use anyhow::Result;

fn main() -> Result<()> {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; 
    let file = fs::read_to_string("array.txt")?;
    let mut values = file
        .split(",")
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<i32>>();
    vec.append(&mut values);
    println!("{:?}", vec);
    Ok(())
}
