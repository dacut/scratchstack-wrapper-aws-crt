use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCThreadDetachState {
    NotCreated = 1,
    Joinable,
    JoinCompleted,
    Managed,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCThreadJoinStrategy {
    Manual = 0,
    Managed,
}

#[repr(C)]
pub struct AwsCThreadOptions {
    pub stack_size: usize,
    pub cpu_id: i32,
    pub join_strategy: AwsCThreadJoinStrategy,
}

pub union AwsCThreadOnce {
    pub ptr: *mut c_void,
}

#[cfg(windows)]
pub type AwsCThreadId = u64;

#[cfg(not(windows))]
pub type AwsCThreadId = libc::pthread_t;

// TODO: Add functions