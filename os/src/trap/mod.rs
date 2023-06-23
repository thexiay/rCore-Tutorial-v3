//! Trap handling functionality
//!
//! For rCore, we have a single trap entry point, namely `__alltraps`. At
//! initialization in [`init()`], we set the `stvec` CSR to point to it.
//!
//! All traps go through `__alltraps`, which is defined in `trap.S`. The
//! assembly language code does just enough work restore the kernel space
//! context, ensuring that Rust code safely runs, and transfers control to
//! [`trap_handler()`].
//!
//! It then calls different functionality based on what exactly the exception
//! was. For example, timer interrupts trigger task preemption, and syscalls go
//! to [`syscall()`].
//! 增加 为内核捕获trap的能力
//! 1. 通过寄存器控制内核中断开关
//! 2. 改写Trap.S，让其能对内核中断和用户态中断做不同处理（内核中断不会换栈）
//! 3. 写一个测试程序，测试在内核态出现时钟中断也能响应

mod context;

use crate::syscall::syscall;
use crate::task::{
    exit_current_and_run_next, 
    suspend_current_and_run_next,
    mark_user_time_end,
    mark_kernel_time_start,
    mark_kernel_time_end,
    mark_user_time_start
};
use crate::timer::set_next_trigger;
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec, sstatus
};

global_asm!(include_str!("trap.S"));

static mut KERNEL_INTERRUPT_TRIGGERED: bool = false;

/// initialize CSR `stvec` as the entry of `__alltraps`
pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

/// timer interrupt enabled
pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

/// 检查内核中断是否触发
pub fn check_kernel_interrupt() -> bool {
    unsafe { (&mut KERNEL_INTERRUPT_TRIGGERED as *mut bool).read_volatile() }
}

/// 标记内核中断已触发
pub fn mark_kernel_interrupt() {
    unsafe {
        (&mut KERNEL_INTERRUPT_TRIGGERED as *mut bool).write_volatile(true);
    }
}

#[no_mangle]
/// handle an interrupt, exception, or system call from user space or interrupt from kernel space
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    match sstatus::read().spp() {
        sstatus::SPP::Supervisor => kernel_trap_handler(cx),
        sstatus::SPP::User => user_trap_handler(cx),
    }
}

/// handle an interrupt, exception, or system call from user space
pub fn user_trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    // case1:从trapp_handler开始到结束表示当前task的kernel占用时间，从结束trap_handler到下一次trap_handler算当前task的user占用时间
    // case2:从trapp_handler开始到run_next_task算当前task的kernel占用时间，从run_next_task的go to user mode到下一次trap_handler算当前task的user占用时间
    mark_user_time_end();
    mark_kernel_time_start();
    let scause = scause::read(); // get trap cause
    let stval = stval::read(); // get extra value
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.", stval, cx.sepc);
            exit_current_and_run_next();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            exit_current_and_run_next();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    mark_kernel_time_end();
    mark_user_time_start();
    cx
}

/// handle an interrupt, exception from kernel space
pub fn kernel_trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            // 内核中断来自一个时钟中断
            println!("kernel interrupt: from timer");
            // 标记一下触发了中断
            mark_kernel_interrupt();
            set_next_trigger();
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            panic!("[kernel] PageFault in kernel, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.", stval, cx.sepc);
        }
        _ => {
            // 其他的内核异常/中断
            panic!("unknown kernel exception or interrupt");
        }
    }
    cx
}

pub use context::TrapContext;
