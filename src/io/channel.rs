use {
    crate::{
        common::{
            AwsCAllocator, AwsCArrayList, AwsCAtomicVar, AwsCCrtStatisticsHandler, AwsCLinkedList, AwsCLinkedListNode,
            AwsCMutex, AwsCTask, AwsCTaskStatus,
        },
        io::{AwsCEventLoop, AwsCIoMessage, AwsCMessagePool},
    },
    std::ffi::c_void,
};

pub type AwsCChannelOnSetupCompletedFn =
    extern "C" fn(channel: *mut AwsCChannel, error_code: i32, user_data: *mut c_void);
pub type AwsCChannelOnShutdownCompletedFn =
    extern "C" fn(channel: *mut AwsCChannel, error_code: i32, user_data: *mut c_void);

#[repr(C)]
pub enum AwsCChannelDirection {
    Read,
    Write,
}

pub type AwsChannelOnSetupCompletedFn =
    extern "C" fn(channel: *mut AwsCChannel, error_code: i32, user_data: *mut c_void);
pub type AwsChannelOnShutdownCompletedFn =
    extern "C" fn(channel: *mut AwsCChannel, error_code: i32, user_data: *mut c_void);

#[repr(C)]
pub struct AwsCChannelSlot {
    pub alloc: *const AwsCAllocator,
    pub channel: *mut AwsCChannel,
    pub adj_left: *mut AwsCChannelSlot,
    pub adj_right: *mut AwsCChannelSlot,
    pub handler: *mut AwsCChannelHandler,
    pub window_size: usize,
    pub upstream_message_overhead: usize,
    pub current_window_update_batch_size: usize,
}

pub type AwsCChannelTaskFn =
    extern "C" fn(channel_task: *mut AwsCChannelTask, arg: *mut c_void, status: AwsCTaskStatus);

#[repr(C)]
pub struct AwsCChannelTask {
    pub wrapper_task: AwsCTask,
    pub task_fn: *const AwsCChannelTaskFn,
    pub arg: *mut c_void,
    pub type_tag: *const u8,
    pub node: AwsCLinkedListNode,
}

#[repr(C)]
pub struct AwsCChannelHandlerVtable {
    pub process_read_message: *const extern "C" fn(
        handler: *mut AwsCChannelHandler,
        slot: *mut AwsCChannelSlot,
        message: *mut AwsCIoMessage,
    ) -> i32,
    pub process_write_message: *const extern "C" fn(
        handler: *mut AwsCChannelHandler,
        slot: *mut AwsCChannelSlot,
        message: *mut AwsCIoMessage,
    ) -> i32,
    pub increment_read_window:
        *const extern "C" fn(handler: *mut AwsCChannelHandler, slot: *mut AwsCChannelSlot, size: usize) -> i32,
    pub shutdown: *const extern "C" fn(
        handler: *mut AwsCChannelHandler,
        slot: *mut AwsCChannelSlot,
        dir: AwsCChannelDirection,
        error_code: i32,
        free_scare_resources_immediately: bool,
    ) -> i32,
    pub initial_window_size: *const extern "C" fn(handler: *mut AwsCChannelHandler) -> usize,
    pub message_overhead: *const extern "C" fn(handler: *mut AwsCChannelHandler) -> usize,
    pub destroy: *const extern "C" fn(handler: *mut AwsCChannelHandler) -> usize,
    pub reset_statistics: *const extern "C" fn(handler: *mut AwsCChannelHandler) -> usize,
    pub gather_statistics:
        *const extern "C" fn(handler: *mut AwsCChannelHandler, stats_list: *mut AwsCArrayList) -> usize,
    pub trigger_read: *const extern "C" fn(handler: *mut AwsCChannelHandler) -> usize,
}

#[repr(C)]
pub struct AwsCChannelHandler {
    pub vtable: *mut AwsCChannelHandlerVtable,
    pub alloc: *const AwsCAllocator,
    pub slot: *mut AwsCChannelSlot,
    pub r#impl: *mut c_void,
}

#[repr(C)]
pub struct AwsCChannelOptions {
    pub event_loop: *mut AwsCEventLoop,
    pub on_setup_completed: *mut AwsCChannelOnSetupCompletedFn,
    pub on_shutdown_completed: *mut AwsCChannelOnShutdownCompletedFn,
    pub setup_user_data: *mut c_void,
    pub shutdown_user_data: *mut c_void,
    pub enable_read_back_pressure: bool,
}

#[repr(C)]
pub struct AwsCChannel {
    // Private implementation
    alloc: *const AwsCAllocator,
    r#loop: *mut AwsCEventLoop,
    first: *mut AwsCChannelSlot,
    msg_pool: *mut AwsCMessagePool,
    channel_sstate: AwsCChannelState,
    shutdown_notify_task: AwsCShutdownNotificationTask,
    on_shutdown_completed: *const AwsCChannelOnShutdownCompletedFn,
    shutdown_user_data: *mut c_void,
    refcount: AwsCAtomicVar,
    deletion_task: AwsCTask,
    statistics_task: AwsCTask,
    statistics_handler: *mut AwsCCrtStatisticsHandler,
    statistics_interval_start_time_ms: u64,
    statistic_list: AwsCArrayList,
    channel_thread_tasks: AwsCChannelThreadTasks,
    cross_thread_tasks: AwsCChanellCrossThreadTasks,
    window_update_batch_emit_threadhold: usize,
    window_update_task: AwsCChannelTask,
    read_back_pressure_enabled: bool,
    window_update_in_progress: bool,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AwsCChannelState {
    AWS_CHANNEL_SETTING_UP,
    AWS_CHANNEL_ACTIVE,
    AWS_CHANNEL_SHUTTING_DOWN,
    AWS_CHANNEL_SHUT_DOWN,
}

#[repr(C)]
struct AwsCShutdownNotificationTask {
    task: AwsCTask,
    error_code: i32,
    slot: *mut AwsCChannelSlot,
    shutdown_immediately: bool,
}

#[repr(C)]
#[allow(dead_code)]
struct AwsCShutdownTask {
    task: AwsCChannelTask,
    channel: *mut AwsCChannel,
    error_code: i32,
    shutdown_immediately: bool,
}

#[repr(C)]
struct AwsCChannelThreadTasks {
    list: AwsCLinkedList,
}

#[repr(C)]
struct AwsCChanellCrossThreadTasks {
    lock: AwsCMutex,
    list: AwsCLinkedList,
    scheduling_task: AwsCTask,
    shutdown_task: AwsCTask,
    is_channel_shut_down: bool,
}
