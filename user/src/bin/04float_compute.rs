#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;


#[no_mangle]
fn main() -> i32 {
    let a = 3.13;
    let b = 2.123;
    println!("a + b = {}", a + b);
    0
}