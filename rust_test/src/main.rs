use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    println!("Files in Directory: ");
    let path = "C:\\Users\\kurtm\\Documents\\Rust";

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        println!("{:?}", entry.file_name().to_str());
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    Ok(())
}
