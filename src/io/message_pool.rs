use {
    crate::{
        common::{AwsCAllocator, AwsCArrayList},
        io::{AwsCIoMessage, AwsCIoMessageType},
    },
    std::ffi::c_void,
};

#[repr(C)]
pub struct AwsCMemoryPool {
    pub alloc: *const AwsCAllocator,
    pub stack: AwsCArrayList,
    pub ideal_sgement_count: u16,
    pub segment_size: usize,
    pub data_ptr: *mut c_void,
}

#[repr(C)]
pub struct AwsCMessagePool {
    pub alloc: *const AwsCAllocator,
    pub appplication_data_pool: AwsCMemoryPool,
    pub small_block_pool: AwsCMemoryPool,
}

#[repr(C)]
pub struct AwsCMessagePoolCreationArgs {
    pub application_data_msg_data_size: usize,
    pub application_data_msg_count: u8,
    pub small_block_msg_data_size: usize,
    pub small_block_msg_count: u8,
}

#[link(name = "aws-c-io")]
extern "C" {
    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_memory_pool_init(
        mempool: *mut AwsCMemoryPool,
        alloc: *const AwsCAllocator,
        ideal_segment_count: u16,
        segment_size: usize,
    ) -> i32;

    pub fn aws_memory_pool_clean_up(mempool: *mut AwsCMemoryPool);

    #[must_use = "returns memory that must be freed or null"]
    pub fn aws_memory_pool_acquire(mempool: *mut AwsCMemoryPool) -> *mut c_void;

    pub fn aws_memory_pool_release(mempool: *mut AwsCMemoryPool, data: *mut c_void);

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_message_pool_init(
        msg_pool: *mut AwsCMessagePool,
        alloc: *const AwsCAllocator,
        args: *const AwsCMessagePoolCreationArgs,
    ) -> i32;

    pub fn aws_message_pool_clean_up(msg_pool: *mut AwsCMessagePool);

    pub fn aws_message_pool_acquire(
        msg_pool: *mut AwsCMessagePool,
        message_type: AwsCIoMessageType,
        size_hint: usize,
    ) -> *mut AwsCIoMessage;

    pub fn aws_message_pool_release(msg_pool: *mut AwsCMessagePool, message: *mut AwsCIoMessage);
}
