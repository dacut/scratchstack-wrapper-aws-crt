use {
    crate::{
        common::{
            AwsCAllocator, AwsCArrayList, AwsCByteCursor, AwsCLinkedList, AwsCMutex, AwsCRefCount, AwsCString, AwsCTask,
        },
        http::{
            AwsCHttp2Setting, AwsCHttpClientConnectionOptions, AwsCHttpConnection, AwsCHttpConnectionMonitoringOptions,
            AwsCHttpProxyConfig, AwsCHttpProxyOptions, AwsCHttpVersion, CProxyEnvVarSettings,
        },
        io::{
            AwsCChannel, AwsCClientBootstrap, AwsCEventLoop, AwsCIoClockFn, AwsCSocketOptions, AwsCTlsConnectionOptions,
        },
    },
    std::ffi::c_void,
};

pub type AwsCHttpConnectionManagerOnConnectionSetupFn =
    extern "C" fn(connection: *mut AwsCHttpConnection, error_code: i32, user_data: *mut c_void);

pub type AwsCHttpConnectionManagerShutdownCompleteFn = extern "C" fn(user_data: *mut c_void);

#[repr(C)]
pub struct AwsCHttpConnectionManagerMetrics {
    pub available_concurrency: usize,
    pub pending_concurrency_acquires: usize,
    pub leased_concurrency: usize,
}

#[repr(C)]
pub struct AwsCHttpConnectionManagerOptions {
    pub bootstrap: *mut AwsCClientBootstrap,
    pub initial_window_size: usize,
    pub socket_options: *mut AwsCSocketOptions,
    pub tls_connection_options: *mut AwsCTlsConnectionOptions,
    pub http2_prior_knowledge: bool,
    pub monitoring_options: *const AwsCHttpConnectionMonitoringOptions,
    pub host: AwsCByteCursor,
    pub port: u16,
    pub initial_settings_array: *mut AwsCHttp2Setting,
    pub num_initial_settings: usize,
    pub max_closed_streams: usize,
    pub http2_conn_manual_window_management: bool,
    pub proxy_options: *const AwsCHttpProxyOptions,
    pub proxy_ev_settings: *const CProxyEnvVarSettings,
    pub max_connections: usize,
    pub shutdown_complete_user_data: *mut c_void,
    pub shutdown_complete_callback: *const AwsCHttpConnectionManagerShutdownCompleteFn,
    pub enable_read_back_pressure: bool,
    pub max_connection_idle_in_milliseconds: u64,
}

type AwsCHttpConnectionManagerCreateConnectionFn = extern "C" fn(options: *mut AwsCHttpClientConnectionOptions) -> i32;
type AwsCHttpConnectionManagerCloseConnectionFn = extern "C" fn(connection: *mut AwsCHttpConnection);
type AwsCHttpConnectionReleaseConnectionFn = extern "C" fn(connection: *mut AwsCHttpConnection);
type AwsCHttpConnectionIsConnectionAvailableFn = extern "C" fn(connection: *mut AwsCHttpConnection) -> bool;
type AwsCHttpConnectionManagerIsCallersThreadFn = extern "C" fn(channel: *mut AwsCChannel) -> bool;
type AwsCHttpConnectionManagerConnectionGetChannelFn =
    extern "C" fn(connection: *mut AwsCHttpConnection) -> *mut AwsCChannel;
type AwsCHttpConnectionManagerConnectionGetVersionFn =
    extern "C" fn(conection: *const AwsCHttpConnection) -> AwsCHttpVersion;

#[repr(C)]
struct AwsCHttpConnectionManagerSystemVtable {
    create_connection: *const AwsCHttpConnectionManagerCreateConnectionFn,
    close_connection: *const AwsCHttpConnectionManagerCloseConnectionFn,
    release_connection: *const AwsCHttpConnectionReleaseConnectionFn,
    is_connection_available: *const AwsCHttpConnectionIsConnectionAvailableFn,
    get_monotonic_time: *const AwsCIoClockFn,
    is_callers_thread: *const AwsCHttpConnectionManagerIsCallersThreadFn,
    connection_get_channel: *const AwsCHttpConnectionManagerConnectionGetChannelFn,
    connection_get_version: *const AwsCHttpConnectionManagerConnectionGetVersionFn,
}

#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum AwsCHttpConnectionManagerStateType {
    AWS_HCMST_UNINITIALIZED,
    AWS_HCMST_READY,
    AWS_HCMST_SHUTTING_DOWN,
}

#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum AwsCHttpConnectionManagerCountType {
    AWS_HCMCT_VENDED_CONNECTION,
    AWS_HCMCT_PENDING_CONNECTIONS,
    AWS_HCMCT_OPEN_CONNECTION,
    AWS_HCMCT_COUNT,
}

#[repr(C)]
pub struct AwsCHttpConnectionManager {
    // Private implementation
    allocator: *const AwsCAllocator,
    system_vtable: *const AwsCHttpConnectionManagerSystemVtable,
    shutdown_complete_callback: *const AwsCHttpConnectionManagerShutdownCompleteFn,
    shutdown_complete_user_data: *mut c_void,
    lock: AwsCMutex,
    state: AwsCHttpConnectionManagerStateType,
    idle_connection_count: usize,
    idle_connections: AwsCLinkedList,
    pending_acquisitions: AwsCLinkedList,
    pending_acquisition_count: usize,
    internal_ref: [usize; AwsCHttpConnectionManagerCountType::AWS_HCMCT_COUNT as usize],
    pending_settings_count: usize,
    bootstrap: AwsCClientBootstrap,
    initial_window_size: usize,
    socket_options: AwsCSocketOptions,
    tls_connection_options: *mut AwsCTlsConnectionOptions,
    proxy_config: *mut AwsCHttpProxyConfig,
    monitoring_options: AwsCHttpConnectionMonitoringOptions,
    host: *mut AwsCString,
    proxy_ev_settings: CProxyEnvVarSettings,
    proxy_ev_tls_options: *mut AwsCTlsConnectionOptions,
    port: u16,
    http2_prior_knowledge: bool,
    initial_settings: *mut AwsCArrayList,
    max_closed_streams: usize,
    http2_conn_manual_window_management: bool,
    max_connections: usize,
    external_ref_count: usize,
    internal_ref_count: AwsCRefCount,
    enable_read_back_pressure: bool,
    max_connection_idle_in_milliseconds: u64,
    cull_task: *mut AwsCTask,
    cull_event_loop: *mut AwsCEventLoop,
}

// TODO: Add functions
