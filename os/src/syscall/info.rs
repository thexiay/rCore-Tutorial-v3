use log::info;
use crate::batch::app_info;

pub fn sys_info_task() -> isize {
    let num = app_info();
    info!("current task num: {}, task name: task_{}", num, num);
    0
}