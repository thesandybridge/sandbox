use std::io::{self, BufRead};
use std::net::Ipv4Addr;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();

    stdin_lock.lines().for_each(|line| {
        if let Ok(line) = line {
            if !line.is_empty() {
                match line.parse::<Ipv4Addr>() {
                    Ok(ip) => {
                        if !is_rfc3330_bogon(ip) {
                            println!("{}", line);
                        }
                    }
                    Err(_) => {
                        eprintln!("Invalid IP address: {}", line);
                    }
                }
            }
        }
    });

    Ok(())
}

fn is_rfc3330_bogon(ip: Ipv4Addr) -> bool {
    match ip.octets() {
        [0, ..] => true,
        [10, ..] => true,
        [127, ..] => true,
        [169, 254, ..] => true,
        [172, second_octet, ..] if second_octet >= 16 && second_octet <= 31 => true,
        [192, 0, 0, ..] => true,
        [192, 0, 2, ..] => true,
        [192, 88, 99, ..] => true,
        [192, 168, ..] => true,
        [198, 18..=19, ..] => true,
        [198, 51, 100, ..] => true,
        [203, 0, 113, ..] => true,
        [224..=239, ..] => true,   // 224.0.0.0 to 239.255.255.255
        [240..=255, ..] => true,   // 240.0.0.0 to 255.255.255.255
        _ => false,
    }
}
