use {
    crate::{
        common::{
            AwsCAllocator, AwsCArrayList, AwsCAtomicVar, AwsCHashTable, AwsCRefCount, AwsCShutdownCallbackOptions,
            AwsCTask, AwsCThreadOptions,
        },
        io::{AwsCIoClockFn, AwsCIoHandle},
    },
    std::ffi::c_void,
};

pub type AwsCEventLoopOnEventFn =
    extern "C" fn(event_loop: *mut AwsCEventLoop, handle: *mut AwsCIoHandle, events: i32, user_data: *mut c_void);

#[repr(C)]
pub struct AwsCEventLoopVtable {
    pub destroy: *mut extern "C" fn(*mut AwsCEventLoop),
    pub run: *mut extern "C" fn(*mut AwsCEventLoop) -> i32,
    pub stop: *mut extern "C" fn(*mut AwsCEventLoop) -> i32,
    pub wait_for_stop_completion: *mut extern "C" fn(event_loop: *mut AwsCEventLoop) -> i32,
    pub schedule_task_now: *mut extern "C" fn(event_loop: *mut AwsCEventLoop, task: *mut AwsCTask) -> i32,
    pub schedule_task_future:
        *mut extern "C" fn(event_loop: *mut AwsCEventLoop, task: *mut AwsCTask, run_at_nanos: u64) -> i32,
    pub cancel_task: *mut extern "C" fn(event_loop: *mut AwsCEventLoop, task: *mut AwsCTask) -> i32,

    #[cfg(feature = "aws-use-io-completion-ports")]
    pub connect_to_io_compltion_port:
        *mut extern "C" fn(event_loop: *mut AwsCEventLoop, handle: *mut AwsCIoHandle) -> i32,

    #[cfg(not(feature = "aws-use-io-completion-ports"))]
    pub subscribe_to_io_events: *mut extern "C" fn(
        event_loop: *mut AwsCEventLoop,
        handle: *mut AwsCIoHandle,
        events: i32,
        on_event: *mut AwsCEventLoopOnEventFn,
        user_data: *mut c_void,
    ) -> i32,

    pub unsubscribe_from_io_events:
        *mut extern "C" fn(event_loop: *mut AwsCEventLoop, handle: *mut AwsCIoHandle) -> i32,
    pub free_io_event_resources: *mut extern "C" fn(user_data: *mut c_void),
    pub is_on_callers_thread: *mut extern "C" fn(event_loop: *mut AwsCEventLoop) -> bool,
}

#[repr(C)]
pub struct AwsCEventLoop {
    pub vtable: *mut AwsCEventLoopVtable,
    pub alloc: *const AwsCAllocator,
    pub clock: *mut AwsCIoClockFn,
    pub local_data: AwsCHashTable,
    pub current_load_factor: AwsCAtomicVar,
    pub latest_tick_start: u64,
    pub current_tick_latency_sum: usize,
    pub next_flush_time: AwsCAtomicVar,
    pub impl_data: *mut c_void,
}

pub type AwsCEventLoopOnLocalObjectRemovedFn = extern "C" fn(obj: *mut AwsCEventLoopLocalObject);

#[repr(C)]
pub struct AwsCEventLoopLocalObject {
    key: *const c_void,
    object: *mut c_void,
    on_object_removed: *mut AwsCEventLoopOnLocalObjectRemovedFn,
}

#[repr(C)]
pub struct AwsCEventLoopOptions {
    clock: *mut AwsCIoClockFn,
    thread_options: *mut AwsCThreadOptions,
}

pub type AwsCNewEventLoopFn = extern "C" fn(
    alloc: *const AwsCAllocator,
    options: *const AwsCEventLoopOptions,
    new_loop_user_data: *mut c_void,
) -> *mut AwsCEventLoop;

#[repr(C)]
pub struct AwsCEventLoopGroup {
    pub allocator: *const AwsCAllocator,
    pub event_loops: AwsCArrayList,
    pub ref_count: AwsCRefCount,
    pub shutdown_options: AwsCShutdownCallbackOptions,
}
