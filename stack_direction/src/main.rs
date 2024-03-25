//fn stack_growth_direction() {
//    let x = 0;
//    let y = 0;
//    println!("Address of x: {:p}", &x);
//    println!("Address of y: {:p}", &y);
//
//    if &y < &x {
//        println!("The stack grows upwards.");
//    } else {
//        println!("The stack grows downwards.");
//    }
//}

fn up_or_down_addr(other: Option<&i32>, depth: i32) -> bool {
    let x = 0;

    match other {
        Some(next_ref) => {
            println!("[{:p}]", &x);
            println!("[{:p}]", next_ref);

            let is_up = &x as *const i32 > next_ref as *const i32;

            if is_up {
                println!("The stack grows upwards.");
            } else {
                println!("The stack grows downwards.");
            }

            is_up
        }
        None if depth > 0 => up_or_down_addr(Some(&x), depth - 1),
        _ => {
            println!("Only one address to compare: [{:p}]", &x);
            false
        }
    }
}

fn main() {
    up_or_down_addr(None, 0);
}
