use {
    crate::common::{AwsCAllocator, AwsCArrayList},
    std::ffi::c_void,
};

pub type AwsCPriorityQueueCompareFn = extern "C" fn(a: *const c_void, b: *const c_void) -> i32;

#[repr(C)]
pub struct AwsCPriorityQueue {
    pub pred: *mut AwsCPriorityQueueCompareFn,
    pub container: AwsCArrayList,
    pub backpointers: AwsCArrayList,
}

#[repr(C)]
pub struct AwsCPriorityQueueNode {
    current_index: usize,
}

#[link(name = "aws-c-common")]
extern "C" {
    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_priority_queue_init_dynamic(
        queue: *mut AwsCPriorityQueue,
        alloc: *const AwsCAllocator,
        default_size: usize,
        item_size: usize,
        pred: *mut AwsCPriorityQueueCompareFn,
    ) -> i32;

    pub fn aws_priority_queue_init_static(
        queue: *mut AwsCPriorityQueue,
        heap: *mut c_void,
        item_count: usize,
        item_size: usize,
        pred: *mut AwsCPriorityQueueCompareFn,
    );

    pub fn aws_priority_queue_backpointer_index_valid(queue: *const AwsCPriorityQueue, index: usize) -> bool;

    pub fn aws_priority_queue_backpointers_valid_deep(queue: *const AwsCPriorityQueue) -> bool;

    pub fn aws_priority_queue_backpointers_valid(queue: *const AwsCPriorityQueue) -> bool;

    pub fn aws_priority_queue_is_valid(queue: *const AwsCPriorityQueue) -> bool;

    pub fn aws_priority_queue_clean_up(queue: *mut AwsCPriorityQueue);

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_priority_queue_push(queue: *mut AwsCPriorityQueue, item: *mut c_void) -> i32;

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_priority_queue_push_ref(
        queue: *mut AwsCPriorityQueue,
        item: *mut c_void,
        backpointer: *mut AwsCPriorityQueueNode,
    ) -> i32;

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_priority_queue_pop(queue: *mut AwsCPriorityQueue, item: *mut c_void) -> i32;

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_priority_queue_remove(
        queue: *mut AwsCPriorityQueue,
        item: *mut c_void,
        node: *const AwsCPriorityQueueNode,
    ) -> i32;

    #[must_use = "returns an i32 that contains a result code (AWS_OP_SUCCESS or AWS_OP_ERR)"]
    pub fn aws_priority_queue_top(queue: *const AwsCPriorityQueue, item: *mut *mut c_void) -> i32;

    pub fn aws_priority_queue_size(queue: *const AwsCPriorityQueue) -> usize;

    pub fn aws_priority_queue_capacity(queue: *const AwsCPriorityQueue) -> usize;
}
