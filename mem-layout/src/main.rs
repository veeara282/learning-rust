use std::mem::{size_of, size_of_val};

fn main() {
    println!("Size of i32: {}", size_of::<i32>());
    println!("Size of Option<i32>: {}", size_of::<Option<i32>>());
    println!("Size of None: Option<i32>: {}", size_of_val::<Option<i32>>(&None));
    println!("Size of Some(1): Option<i32>: {}", size_of_val::<Option<i32>>(&Some(1)));
    println!("Size of None: Option<i8>: {}", size_of_val::<Option<i8>>(&None));
    println!("Size of Some(1): Option<i8>: {}", size_of_val::<Option<i8>>(&Some(1)));
    println!("Size of &i32: {}", size_of::<&i32>());
    println!("Size of Option<&i32>: {}", size_of::<Option<&i32>>());
    println!("Size of None: Option<&i32>: {}", size_of_val::<Option<&i32>>(&None));
    println!("Size of Some(&1): Option<&i32>: {}", size_of_val::<Option<&i32>>(&Some(&1)));
}
