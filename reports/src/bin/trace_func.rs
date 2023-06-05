
#![feature(backtrace)]
use std::backtrace::Backtrace;

//实现一个linux应用程序B，能打印出调用栈链信息。
fn main() {
    let backtrace = Backtrace::capture();
    println!("{:?}", backtrace);
}