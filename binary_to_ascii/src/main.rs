use anyhow::Result;
use std::fs;

enum Int32orUInt32 {
    Int32(i32),
    UInt32(u32),
}

fn u32_to_negative_i32(value: u32) -> i32 {
    !(value as i32) + 1
}

fn binary_to_decimal(input: &str, negative: bool) -> Int32orUInt32 {
    let mut decimal_value: u32 = 0;
    let base: u32 = 2;
    let len = input.len();

    for (i, c) in input.chars().rev().enumerate() {
        if c == '1' {
            decimal_value += base.pow(i as u32);
        }
    }

    if negative && input.chars().next() == Some('1') && len == 32 {
        Int32orUInt32::Int32(u32_to_negative_i32(decimal_value))
    } else {
        Int32orUInt32::UInt32(decimal_value)
    }
}

fn main() -> Result<()> {
    let binary_file = fs::read_to_string("./binary")?;
    let results: Vec<Int32orUInt32> = binary_file
        .lines()
        .map(|l| binary_to_decimal(l, true))
        .collect();

    for result in results {
        match result {
            Int32orUInt32::Int32(val) => println!("Signed: -{}", val),
            Int32orUInt32::UInt32(val) => println!("Unsigned: {}", val),
        }
    }

    Ok(())
}
