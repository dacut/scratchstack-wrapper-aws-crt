mod allocator;
mod array_list;
mod atomics;
mod byte_buf;
mod date_time;
mod error;
mod hash_table;
mod linked_list;
mod logging;
mod mutex;
mod package;
mod priority_queue;
mod ref_count;
mod statistics;
mod string;
mod task_scheduler;
mod thread;

pub use {
    allocator::*, array_list::*, atomics::*, byte_buf::*, date_time::*, error::*, hash_table::*, linked_list::*,
    logging::*, mutex::*, package::*, priority_queue::*, ref_count::*, statistics::*, string::*, task_scheduler::*,
    thread::*,
};

#[link(name = "aws-c-common")]
extern "C" {
    pub fn aws_common_library_init(allocator: *const AwsCAllocator);
    pub fn aws_common_library_clean_up();
    pub fn aws_common_fatal_assert_library_initialized();
}
