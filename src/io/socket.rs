use {
    crate::{
        common::{AwsCAllocator, AwsCByteBuf, AwsCByteCursor},
        io::{AwsCChannelHandler, AwsCChannelDirection, AwsCEventLoop, AwsCIoHandle},
    },
    std::ffi::c_void,
};

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCSocketDomain {
    AWS_SOCKET_IPV4,
    AWS_SOCKET_IPV6,
    AWS_SOCKET_LOCAL,
    AWS_SOCKET_VSOCK,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCSocketType {
    AWS_SOCKET_STREAM,
    AWS_SOCKET_DGRAM,
}

#[repr(C)]
pub struct AwsCSocketOptions {
    pub r#type: AwsCSocketType,
    pub domain: AwsCSocketDomain,
    pub connect_timeout_ms: u32,
    pub keep_alive_interval_sec: u16,
    pub keep_alive_timeout_sec: u16,
    pub keep_alive_max_failed_probes: u16,
    pub keep_alive: bool,
}

pub type AwsCSocketOnConnectionResultFn =
    extern "C" fn(socket: *mut AwsCSocket, error_code: i32, user_data: *mut c_void);

pub type AwsCSocketOnAcceptResultFn =
    extern "C" fn(socket: *mut AwsCSocket, error_code: i32, new_socket: *mut AwsCSocket, user_data: *mut c_void);

pub type AwsCSocketOnWriteCompletedFn =
    extern "C" fn(socket: *mut AwsCSocket, error_code: i32, bytes_written: usize, user_data: *mut c_void);

pub type AwsCSocketOnReadableFn = extern "C" fn(socket: *mut AwsCSocket, error_code: i32, user_data: *mut c_void);

#[cfg(target_os = "windows")]
pub const AWS_ADDRESS_MAX_LEN: usize = 256;

#[cfg(any(target_os = "android", target_os = "linux"))]
pub const AWS_ADDRESS_MAX_LEN: usize = 108;

#[cfg(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub const AWS_ADDRESS_MAX_LEN: usize = 104;

#[repr(C)]
pub struct AwsCSocketEndpoint {
    pub address: [u8; AWS_ADDRESS_MAX_LEN],
    pub port: u16,
}

#[repr(C)]
pub struct AwsCSocket {
    pub allocator: *const AwsCAllocator,
    pub local_endpoint: AwsCSocketEndpoint,
    pub remote_endpoint: AwsCSocketEndpoint,
    pub options: AwsCSocketOptions,
    pub io_handle: AwsCIoHandle,
    pub event_loop: *mut AwsCEventLoop,
    pub handler: *mut AwsCChannelHandler,
    pub state: i32,
    pub readable_fn: *const AwsCSocketOnReadableFn,
    pub readable_user_data: *mut c_void,
    pub connection_result_fn: *const AwsCSocketOnConnectionResultFn,
    pub accept_result_fn: *const AwsCSocketOnAcceptResultFn,
    pub connect_accept_user_data: *mut c_void,
    pub r#impl: *mut c_void,
}

#[cfg(windows)]
pub type AwsCMsFnPtr = *const extern "C" fn();

#[link(name = "aws-c-io")]
extern "C" {
    #[cfg(windows)]
    pub fn aws_check_and_init_winsock();

    #[cfg(windows)]
    pub fn aws_winsock_get_connectex_fn() -> AwsCMsFnPtr;

    #[cfg(windows)]
    pub fn aws_winsock_get_acceptex_fn() -> AwsCMsFnPtr;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_socket_init(
        socket: *mut AwsCSocket,
        alloc: *const AwsCAllocator,
        options: *const AwsCSocketOptions,
    ) -> i32;

    pub fn aws_socket_clean_up(socket: *mut AwsCSocket);

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_socket_connect(socket: *mut AwsCSocket, remote_endpoint: *const AwsCSocketEndpoint, event_loop: *mut AwsCEventLoop, on_connection_result: *const AwsCSocketOnConnectionResultFn, user_data: *mut c_void) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_socket_bind(socket: *mut AwsCSocket, local_endpoint: *const AwsCSocketEndpoint) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_socket_get_bound_address(socket: *const AwsCSocket, out_address: *mut AwsCSocketEndpoint) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_socket_listen(socket: *mut AwsCSocket, max_backlog_size: i32) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_socket_start_accept(socket: *mut AwsCSocket, event_loop: *mut AwsCEventLoop, on_accept_result: *const AwsCSocketOnAcceptResultFn, user_data: *mut c_void) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_socket_stop_accept(socket: *mut AwsCSocket) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_socket_close(socket: *mut AwsCSocket) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_socket_shutdown(socket: *mut AwsCSocket, dir: AwsCChannelDirection) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_socket_set_options(socket: *mut AwsCSocket, options: *const AwsCSocketOptions) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_socket_assign_to_event_loop(socket: *mut AwsCSocket, event_loop: *mut AwsCEventLoop) -> i32;

    pub fn aws_socket_get_event_loop(socket: *mut AwsCSocket) -> *mut AwsCEventLoop;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_socket_subscribe_to_readable_events(socket: *mut AwsCSocket, on_readable: *const AwsCSocketOnReadableFn, user_data: *mut c_void) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_socket_read(socket: *mut AwsCSocket, buffer: *mut AwsCByteBuf, amount_read: *mut usize) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_socket_write(socket: *mut AwsCSocket, cursor: *const AwsCByteCursor, written_fn: *const AwsCSocketOnWriteCompletedFn, user_data: *mut c_void) -> i32;

    #[must_use = "returns an i32 that contains a result code"]
    pub fn aws_socket_get_error(socket: *mut AwsCSocket) -> i32;

    pub fn aws_socket_is_open(socket: *mut AwsCSocket) -> bool;
}
