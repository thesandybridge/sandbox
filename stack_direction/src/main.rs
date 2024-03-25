//fn up_or_down(other: Option<&i32>) -> bool {
//    let x = 0;
//    let x_ptr: *const i32 = &x;
//
//    match other {
//        Some(other_ref) => x_ptr > other_ref as *const i32,
//        None => false,
//    }
//}

fn up_or_down_addr(other: Option<&i32>) -> bool {
    let x = 0;
    let x_ptr: *const i32 = &x;

    println!("[{:p}]", x_ptr);

    match other {
        Some(other_ref) => {
            println!("[{:p}]", other_ref as *const i32);

            x_ptr > other_ref as *const i32
        }
        None => false,
    }
}

fn main() {
    let y = 0;
    // wanted to adjust to show the address
    let answer_with_address = if up_or_down_addr(Some(&y)) {
        "up"
    } else {
        "down"
    };
    println!("Stack direction: {}", answer_with_address);
    println!("Architecture: {}", std::env::consts::ARCH);

    //let answer_without_address = if up_or_down(None) { "up" } else { "down" };
    //println!("Result without address: {}", answer_without_address);
}
