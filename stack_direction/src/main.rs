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
        Some(other_ref) => {
            println!("Current addr: {:p} Previous addr: {:p}", &x, other_ref);
            &x as *const i32 > other_ref as *const i32
        }
        None if depth > 0 => up_or_down_addr(Some(&x), depth - 1),
        _ => false,
    }
}

fn main() {
    let is_up = up_or_down_addr(None, 10);

    if is_up {
        println!("The stack grows upwards.");
    } else {
        println!("The stack grows downwards.");
    }
}
