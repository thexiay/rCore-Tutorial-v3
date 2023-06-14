use log::error;

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if !range_check(buf, len) {
        return -1;
    }
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        },
        _ => {
            error!("Unsupported fd: {} in sys_write!", fd);
            -1
        }
    }
}

fn range_check(buf: *const u8, len: usize) -> bool {
    // 思路：每个用户程序可用的地址空间是固定的，/os/src/batch.rs中分配好了user stack的空间，这里做校验即可
    use crate::batch::{app_stack_range, app_address_range};
    let (stack_top, stack_bottom) = app_stack_range();
    let (app_bottom, app_top) = app_address_range();
    if ((buf as usize) >= stack_top && (buf as usize + len) < stack_bottom) 
            || ((buf as usize) >= app_bottom && (buf as usize + len) < app_top) {
        true
    } else {
        error!("illegal buffer address: ({:#x}, {:#x}), legal buffer address is in stack({:#x}, {:#x}) or in app({:#x}, {:#x})", 
            buf as usize,
            buf as usize + len,
            stack_top,
            stack_bottom,
            app_bottom,
            app_top);
        false
    }
}