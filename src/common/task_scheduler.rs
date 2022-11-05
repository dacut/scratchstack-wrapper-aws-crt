use {
    std::ffi::c_void,
    crate::common::{AwsCAllocator, AwsCLinkedList, AwsCLinkedListNode, AwsCPriorityQueue, AwsCPriorityQueueNode},
};

pub type AwsCTaskFn = extern "C" fn(task: *mut AwsCTask, arg: *mut c_void, status: AwsCTaskStatus);

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AwsCTaskStatus {
    RunReady,
    Canceled,
}

#[repr(C)]
pub union AwsCTaskAbiExt {
    pub scheduled: bool,
    pub reserved: usize,
}

#[repr(C)]
pub struct AwsCTask {
    pub r#fn: *mut AwsCTaskFn,
    pub arg: *mut c_void,
    pub timestamp: u64,
    pub node: AwsCLinkedListNode,
    pub priority_queue_node: AwsCPriorityQueueNode,
    pub type_tag: *const u8,
    pub abi_extension: AwsCTaskAbiExt,
}

#[repr(C)]
pub struct AwsCTaskScheduler {
    pub alloc: *const AwsCAllocator,
    pub timed_queue: AwsCPriorityQueue,
    pub timed_list: AwsCLinkedList,
    pub asap_list: AwsCLinkedList,
}

#[link(name = "aws-c-common")]
extern "C" {
    pub fn aws_task_init(task: *mut AwsCTask, r#fn: *mut AwsCTaskFn, arg: *mut c_void, type_tag: *const u8);
    pub fn aws_task_run(task: *mut AwsCTask, status: AwsCTaskStatus);

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_task_scheduler_init(scheduler: *mut AwsCTaskScheduler, alloc: *const AwsCAllocator) -> i32;

    pub fn aws_task_scheduler_clean_up(scheduler: *mut AwsCTaskScheduler);

    pub fn aws_task_scheduler_is_valid(scheduler: *const AwsCTaskScheduler) -> bool;

    pub fn aws_task_scheduler_has_tasks(scheduler: *const AwsCTaskScheduler, next_task_time: *mut u64) -> bool;

    pub fn aws_task_scheduler_schedule_now(scheduler: *mut AwsCTaskScheduler, task: *mut AwsCTask);

    pub fn aws_task_scheduler_schedule_future(scheduler: *mut AwsCTaskScheduler, task: *mut AwsCTask, time_to_run: u64);

    pub fn aws_task_scheduler_cancel_task(scheduler: *mut AwsCTaskScheduler, task: *mut AwsCTask);

    pub fn aws_task_scheduler_run_all(scheduler: *mut AwsCTaskScheduler, current_time: u64);

    pub fn aws_task_status_to_c_str(status: AwsCTaskStatus) -> *const u8;   
}
