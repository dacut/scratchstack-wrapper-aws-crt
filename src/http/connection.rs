use {
    crate::{
        common::{AwsCAllocator, AwsCArrayList, AwsCAtomicVar, AwsCByteCursor, AwsCHashTable, AwsCMutex},
        http::{
            AwsCHttpMakeRequestOptions, AwsCHttpMessage, AwsCHttpOnIncomingRequestFn,
            AwsCHttpOnServerConnectionShutdownFn, AwsCHttpProxyOptions, AwsCHttpRequestHandlerOptions,
            AwsCHttpServerOnDestroyFn, AwsCHttpServerOnIncomingConnectionFn, AwsCHttpStream, AwsCHttpVersion,
            CProxyEnvVarSettings,
        },
        io::{
            AwsCChannelHandler, AwsCChannelHandlerVtable, AwsCChannelSlot, AwsCClientBootstrap, AwsCEventLoop,
            AwsCServerBootstrap, AwsCSocket, AwsCSocketOptions, AwsCTlsConnectionOptions,
        },
    },
    std::{ffi::c_void, mem::ManuallyDrop},
};

type AwsCHttpProxyRequestTransformFn = extern "C" fn(request: *mut AwsCHttpMessage, user_data: *mut c_void) -> i32;

#[repr(C)]
struct AwsCHttpConnectionVtable {
    channel_handler_vtable: AwsCChannelHandlerVtable,
    on_channel_handler_installed: *const extern "C" fn(handler: *mut AwsCChannelHandler, slot: *mut AwsCChannelSlot),
    make_request: *const extern "C" fn(
        client_connection: *mut AwsCHttpConnection,
        options: *const AwsCHttpMakeRequestOptions,
    ) -> *mut AwsCHttpStream,
    new_server_request_handler_stream:
        *const extern "C" fn(options: *const AwsCHttpRequestHandlerOptions) -> *mut AwsCHttpStream,
    stream_send_response: *const extern "C" fn(stream: *mut AwsCHttpStream, response: *mut AwsCHttpMessage) -> i32,
    close: *const extern "C" fn(connection: *mut AwsCHttpConnection),
    stop_new_request: *const extern "C" fn(connection: *const AwsCHttpConnection),
    is_open: *const extern "C" fn(connection: *const AwsCHttpConnection) -> bool,
    new_requests_allowed: *const extern "C" fn(connection: *const AwsCHttpConnection) -> bool,
    update_window: *const extern "C" fn(connection: *mut AwsCHttpConnection, increment_size: u32),
    change_settings: *const extern "C" fn(
        http2_connection: *mut AwsCHttpConnection,
        settings_array: *const AwsCHttp2Setting,
        num_settings: usize,
        on_completed: *const AwsCHttp2OnChangeSettingsCompleteFn,
        user_data: *mut c_void,
    ) -> i32,
    send_ping: *const extern "C" fn(
        http2_connection: *mut AwsCHttpConnection,
        optional_opaque_data: *const AwsCByteCursor,
        on_completed: *const AwsCHttp2OnPingCompleteFn,
        user_data: *mut c_void,
    ) -> i32,
    send_goaway: *const extern "C" fn(
        http2_connection: *mut AwsCHttpConnection,
        out_http2_error: *mut u32,
        out_last_stream_id: *mut u32,
    ) -> i32,
    get_received_goaway: *const extern "C" fn(
        http2_connection: *const AwsCHttpConnection,
        out_http2_error: *mut u32,
        out_last_stream_id: *mut u32,
    ) -> i32,
    get_local_settings:
        *const extern "C" fn(http2_connection: *const AwsCHttpConnection, out_settings: *mut AwsCHttp2Setting),
    get_remote_settings:
        *const extern "C" fn(http2_connection: *const AwsCHttpConnection, out_settings: *mut AwsCHttp2Setting),
}

#[repr(C)]
pub struct AwsCHttpConnection {
    // Private implementation
    vtable: *const AwsCHttpConnectionVtable,
    channel_handler: AwsCChannelHandler,
    channel_slot: *mut AwsCChannelSlot,
    alloc: *const AwsCAllocator,
    http_version: AwsCHttpVersion,
    proxy_request_transform: *const AwsCHttpProxyRequestTransformFn,
    user_data: *mut c_void,
    refcount: AwsCAtomicVar,
    next_stream_id: u32,
    client_or_server_data: CHttpConnectionClientOrServerData,
    client_data: *mut AwsCHttpConnectionClientData,
    server_data: *mut AwsCHttpConnectionServerData,
}

#[repr(C)]
union CHttpConnectionClientOrServerData {
    client: ManuallyDrop<AwsCHttpConnectionClientData>,
    server: ManuallyDrop<AwsCHttpConnectionServerData>,
}

#[repr(C)]
struct AwsCHttpConnectionClientData {
    delete_me: u8,
}

#[repr(C)]
struct AwsCHttpConnectionServerData {
    on_incoming_request: *const AwsCHttpOnIncomingRequestFn,
    on_shutdown: *const AwsCHttpOnServerConnectionShutdownFn,
}

pub type AwsCHttpOnClientConnectionSetupFn =
    extern "C" fn(connection: *mut AwsCHttpConnection, error_code: i32, user_data: *mut c_void);

pub type AwsCHttpOnClientConnectionShutdownFn =
    extern "C" fn(connection: *mut AwsCHttpConnection, error_code: i32, user_data: *mut c_void);

pub type AwsCHttp2OnChangeSettingsCompleteFn =
    extern "C" fn(http2_connection: *mut AwsCHttpConnection, error_code: i32, user_data: *mut c_void);

pub type AwsCHttp2OnPingCompleteFn = extern "C" fn(
    http2_connection: *mut AwsCHttpConnection,
    round_trip_time_ns: u64,
    error_code: i32,
    user_data: *mut c_void,
);

pub type AwsCHttp2OnGoAwayReceivedFn = extern "C" fn(
    http2_connection: *mut AwsCHttpConnection,
    last_stream_id: u32,
    http2_error_code: u32,
    debug_data: AwsCByteCursor,
    user_data: *mut c_void,
);

pub type AwsCHttp2OnRemoveSettingsChangeFn = extern "C" fn(
    http2_connection: *mut AwsCHttpConnection,
    settings_array: *const AwsCHttp2Setting,
    num_settings: usize,
    user_data: *mut c_void,
);

pub type AwsCHttpStatisticsObserverFn =
    extern "C" fn(connection_nonce: usize, stats_list: *const AwsCArrayList, user_data: *mut c_void);

#[repr(C)]
pub struct AwsCHttpConnectionMonitoringOptions {
    pub minimum_throughput_bytes_per_second: u64,
    pub allowable_throughput_failure_interval_seconds: u32,
    pub statistics_observer_fn: AwsCHttpStatisticsObserverFn,
    pub statistics_observer_user_data: *mut c_void,
}

#[repr(C)]
pub struct AwsHttp1ConnectionOptions {
    pub read_buffer_capacity: usize,
}

#[repr(C)]
pub struct AwsHttp2ConnectionOptions {
    pub initial_settings_array: *mut AwsCHttp2Setting,
    pub num_initial_settings: usize,
    pub on_initial_settings_completed: *const AwsCHttp2OnChangeSettingsCompleteFn,
    pub max_closed_streams: usize,
    pub on_goaway_received: *const AwsCHttp2OnGoAwayReceivedFn,
    pub on_remove_settings_change: *const AwsCHttp2OnRemoveSettingsChangeFn,
    pub conn_manual_window_management: bool,
}

#[repr(C)]
pub struct AwsCHttpClientConnectionOptions {
    pub self_size: usize,
    pub allocator: *const AwsCAllocator,
    pub bootstrap: *mut AwsCClientBootstrap,
    pub host_name: AwsCByteCursor,
    pub port: u16,
    pub socket_options: *const AwsCSocketOptions,
    pub tls_options: *const AwsCTlsConnectionOptions,
    pub proxy_options: *const AwsCHttpProxyOptions,
    pub proxy_ev_settings: *const CProxyEnvVarSettings,
    pub monitoring_options: *const AwsCHttpConnectionMonitoringOptions,
    pub manual_window_management: bool,
    pub initial_window_size: usize,
    pub user_data: *mut c_void,
    pub on_setup: *const AwsCHttpOnClientConnectionSetupFn,
    pub on_shutdown: *const AwsCHttpOnClientConnectionShutdownFn,
    pub prior_knowledge_http2: bool,
    pub alpn_string_map: *mut AwsCHashTable,
    pub http1_options: *const AwsHttp1ConnectionOptions,
    pub http2_options: *const AwsHttp2ConnectionOptions,
    pub requested_event_loop: *mut AwsCEventLoop,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AwsCHttp2SettingsId {
    AWS_HTTP2_SETTINGS_HEADER_TABLE_SIZE = 0x1,
    AWS_HTTP2_SETTINGS_ENABLE_PUSH = 0x2,
    AWS_HTTP2_SETTINGS_MAX_CONCURRENT_STREAMS = 0x3,
    AWS_HTTP2_SETTINGS_INITIAL_WINDOW_SIZE = 0x4,
    AWS_HTTP2_SETTINGS_MAX_FRAME_SIZE = 0x5,
    AWS_HTTP2_SETTINGS_MAX_HEADER_LIST_SIZE = 0x6,
    AWS_HTTP2_SETTINGS_END_RANGE, /* End of known values */
}

pub const AWS_HTTP2_SETTINGS_BEGIN_RANGE: isize = 0x1;

#[repr(C)]
pub struct AwsCHttp2Setting {
    pub id: AwsCHttp2SettingsId,
    pub value: u32,
}

pub const AWS_HTTP2_DEFAULT_MAX_CLOSED_STREAMS: u32 = 32;
pub const AWS_HTTP2_PING_DATA_SIZE: u32 = 8;
pub const AWS_HTTP2_SETTINGS_COUNT: u32 = 6;

#[repr(C)]
#[allow(dead_code)]
pub struct AwsCHttpServer {
    // Private implementation
    alloc: *const AwsCAllocator,
    bootstrap: *mut AwsCServerBootstrap,
    is_using_tls: bool,
    manual_window_management: bool,
    initial_window_size: usize,
    user_data: *mut c_void,
    on_incoming_connection: *const AwsCHttpServerOnIncomingConnectionFn,
    on_destroy_complete: *const AwsCHttpServerOnDestroyFn,
    socket: *mut AwsCSocket,
    synced_data: AwsCHttpServerSyncedData,
}

#[repr(C)]
struct AwsCHttpServerSyncedData {
    lock: AwsCMutex,
    is_shutting_down: bool,
    channel_to_connection_map: AwsCHashTable,
}

// TOOD: Add functions.
