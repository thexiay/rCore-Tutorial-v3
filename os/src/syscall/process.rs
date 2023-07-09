use crate::config::PAGE_SIZE;
use crate::task::{
    suspend_current_and_run_next,
    exit_current_and_run_next, mmap, munmap,
};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

use crate::mm::{translated_byte_buffer, VirtAddr, MapPermission};
use crate::task::current_user_token;
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let _us = get_time_us();
    // 让指针所指向的那一块区域去填值，需要找到其在应用程序上栈上的虚拟地址对应的物理地址
    // 1.找到当前进程
    // 2.当前进程的页表
    // 3.拿到当前地址的vpn
    // 4.查页表选项，获取ppn，然后加上ppn就是实际地址
    // 5.修改地址中的数据
    let mut byte_buffer = translated_byte_buffer(current_user_token(), _ts, core::mem::size_of::<TimeVal>());
    let ts = TimeVal {
        sec: _us / 1_000_000,
        usec: _us % 1_000_000,
    };
    let bytes: [u8; core::mem::size_of::<TimeVal>()] = unsafe { core::mem::transmute(ts) };
    let mut pos = 0_usize;
    for a in byte_buffer.iter_mut() {
        for b in (*a).iter_mut() {
            *b = bytes[pos];
            pos += 1;
        }
    }
    0
}

// start is start virutal address, len is, prot is prilige
pub fn sys_mmap(start: usize, _len: usize, prot: usize) -> isize {
    let res = mmap(start, _len, prot);
    match res {
        Err(e) => {
            println!("{}", e);
            -1
        },
        Ok(_) => 0,
    }
}

// 可以取消一部分吗？先假定不能取消一部分，该部分内存必须和之前的内存一样
pub fn sys_munmap(start: usize, _len: usize) -> isize {
    let res = munmap(start, _len);
    match res {
        Err(e) => {
            println!("{}", e);
            -1
        },
        Ok(_) => 0,
    }
}