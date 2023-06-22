use crate::timer::get_time_ms;
use log::error;

#[derive(Copy, Clone)]
pub struct TaskMetric {
    pub user_cost_ms: usize,
    pub kernel_cost_ms: usize,
    pub tmp_time_marker: usize,
}

impl TaskMetric {
    
    pub fn mark_user_start(&mut self) {
        self.tmp_time_marker = get_time_ms();
    }
    
    pub fn mark_user_end(&mut self) {
        if self.tmp_time_marker == 0 {
            error!("Exception on user end.Start does not call");
            return 
        }
        self.user_cost_ms = self.user_cost_ms + get_time_ms() - self.tmp_time_marker;
        self.tmp_time_marker = 0;
    }

    pub fn mark_kernel_start(&mut self) {
        self.tmp_time_marker = get_time_ms();
    }

    pub fn mark_kernel_end(&mut self) {
        if self.tmp_time_marker == 0 {
            error!("Exception on kernel end.Start does not call");
            return 
        }
        self.kernel_cost_ms = self.kernel_cost_ms + get_time_ms() - self.tmp_time_marker;
        self.tmp_time_marker = 0;
    }
}