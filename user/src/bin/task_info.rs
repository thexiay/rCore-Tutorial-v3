#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::info_task;

#[no_mangle]
pub fn main() -> i32 {
    let info = info_task();
    assert_eq!(0, info);
    println!("info task message success!");
    0 
}