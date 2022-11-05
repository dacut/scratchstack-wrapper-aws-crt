use {
    crate::{
        common::AwsCAllocator,
        http::{AwsCHttpConnection, AwsCHttpServer, AwsCHttpStream},
        io::{AwsCServerBootstrap, AwsCSocketEndpoint, AwsCSocketOptions, AwsCTlsConnectionOptions},
    },
    std::ffi::c_void,
};

pub type AwsCHttpServerOnIncomingConnectionFn = extern "C" fn(
    server: *mut AwsCHttpServer,
    connection: *mut AwsCHttpConnection,
    error_code: i32,
    user_data: *mut c_void,
);

pub type AwsCHttpServerOnDestroyFn = extern "C" fn(user_data: *mut c_void);

#[repr(C)]
pub struct AwsCHttpServerOptions {
    pub self_size: usize,
    pub allocator: *const AwsCAllocator,
    pub bootstrap: *mut AwsCServerBootstrap,
    pub endpoint: *mut AwsCSocketEndpoint,
    pub socket_options: *mut AwsCSocketOptions,
    pub tls_options: *mut AwsCTlsConnectionOptions,
    pub initial_window_size: usize,
    pub server_user_data: *mut c_void,
    pub on_incoming_connection: *mut AwsCHttpServerOnIncomingConnectionFn,
    pub on_destroy: *mut AwsCHttpServerOnDestroyFn,
    pub manual_window_management: bool,
}

pub type AwsCHttpOnIncomingRequestFn =
    extern "C" fn(connection: *mut AwsCHttpConnection, user_data: *mut c_void) -> *mut AwsCHttpStream;

pub type AwsCHttpOnServerConnectionShutdownFn =
    extern "C" fn(connection: *mut AwsCHttpConnection, error_code: i32, connection_user_data: *mut c_void);

#[repr(C)]
pub struct AwsCHttpServerConnectionOptions {
    pub self_size: usize,
    pub connection_user_data: *mut c_void,
    pub on_incoming_request: *mut AwsCHttpOnIncomingRequestFn,
    pub on_shutdown: *mut AwsCHttpOnServerConnectionShutdownFn,
}

// TODO: Add functions
