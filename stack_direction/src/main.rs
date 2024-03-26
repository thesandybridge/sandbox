use std::mem::size_of;
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

fn get_index_ptr<T>(array: &[T], index: usize) -> i32 {
    unsafe {
        // convert the array to a raw pointer
        let x_ptr = array.as_ptr();
        // calculate the address of the element at index offset
        let index_ptr = x_ptr as usize + (index * size_of::<i32>() as usize);
        // convert the address to a raw pointer
        let index_ptr = index_ptr as *const i32;
        return *index_ptr;
    }
}

fn main() {
    up_or_down_addr(None, 0);
    // make an sequential array of 10 elements
    let x = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let index = get_index_ptr(&x, 2);
    println!("The value at index 2 is: {}", index);
}
