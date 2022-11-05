use {
    crate::{
        common::{
            AwsCAllocator, AwsCArrayList, AwsCByteCursor, AwsCLinkedListNode, AwsCRefCount,
            AwsCShutdownCallbackOptions, AwsCString,
        },
        io::{AwsCEventLoopGroup, AwsCIoClockFn},
    },
    std::ffi::c_void,
};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AwsCAddressRecordType {
    A,
    AAAA,
}

pub type AwsCGetHostAddressFlags = u32;
pub const AWS_GET_HOST_ADDRESS_COUNT_RECORD_TYPE_A: AwsCGetHostAddressFlags = 0x00000001;
pub const AWS_GET_HOST_ADDRESS_COUNT_RECORD_TYPE_AAAA: AwsCGetHostAddressFlags = 0x00000002;

#[repr(C)]
pub struct AwsCHostAddress {
    pub allocator: *const AwsCAllocator,
    pub host: *const AwsCString,
    pub address: *const AwsCString,
    pub record_type: AwsCAddressRecordType,
    pub expiry: u64,
    pub use_count: usize,
    pub connection_failure_count: usize,
    pub weight: u8,
}

pub type AwsCOnHostResolvedResultFn = extern "C" fn(
    resolver: *mut AwsCHostResolver,
    host_name: *const AwsCString,
    err_code: i32,
    host_addresses: *const AwsCArrayList,
    user_data: *mut c_void,
);

pub type AwsCResolveHostImplementationFn = extern "C" fn(
    allocator: *const AwsCAllocator,
    host_name: *const AwsCString,
    output_addresses: *mut AwsCArrayList,
    user_data: *mut c_void,
) -> i32;

#[repr(C)]
pub struct AwsCHostResolutionConfig {
    pub r#impl: *const AwsCResolveHostImplementationFn,
    pub max_ttl: usize,
    pub impl_data: *mut c_void,
}

#[repr(C)]
pub struct AwsCHostListener {
    // Private implementation
    resolver: *mut AwsCHostResolver,
    host_name: *mut AwsCString,
    resolved_address_callback: *const AwsCHostListenerResolvedAddressFn,
    expired_address_callback: *const AwsCHostListenerExpiredAddressFn,
    shutdown_callback: *const AwsCHostListenerShutdownFn,
    user_data: *mut c_void,
    synced_data: AwsCHostListenerSyncedData,
    threaded_data: AwsCHostListenerThreadedData,
}

#[repr(C)]
struct AwsCHostListenerSyncedData {
    node: AwsCLinkedListNode,
    owned_by_resolver_thread_pending_destroy: u32,
}

#[repr(C)]
struct AwsCHostListenerThreadedData {
    node: AwsCLinkedListNode,
    pin_host_entry: bool,
}

pub type AwsCHostListenerResolvedAddressFn =
    extern "C" fn(listener: *mut AwsCHostListener, new_address_list: *const AwsCArrayList, user_data: *mut c_void);

pub type AwsCHostListenerExpiredAddressFn =
    extern "C" fn(listener: *mut AwsCHostListener, expired_address_list: *const AwsCArrayList, user_data: *mut c_void);

pub type AwsCHostListenerShutdownFn = extern "C" fn(user_data: *mut c_void);

#[repr(C)]
pub struct AwsCHostListenerOptions {
    pub host_name: AwsCByteCursor,
    resolved_address_callback: *const AwsCHostListenerResolvedAddressFn,
    expired_address_callback: *const AwsCHostListenerExpiredAddressFn,
    shutdown_callback: *const AwsCHostListenerShutdownFn,
    user_data: *mut c_void,
    pin_hsot_entry: bool,
}

#[repr(C)]
pub struct AwsCHostResolverVtable {
    pub destroy: *const extern "C" fn(resolver: AwsCHostResolver),
    pub resolve_host: *const extern "C" fn(
        resolver: AwsCHostResolver,
        host_name: *const AwsCString,
        res: *const AwsCOnHostResolvedResultFn,
        config: *mut AwsCHostResolutionConfig,
        user_data: *mut c_void,
    ) -> i32,
    pub record_connection_failure:
        *const extern "C" fn(resolver: AwsCHostResolver, address: *mut AwsCHostAddress) -> i32,
    pub purge_cache: *const extern "C" fn(resolver: *mut AwsCHostResolver) -> i32,
    pub get_host_address_count:
        *const extern "C" fn(resolver: *mut AwsCHostResolver, host_name: *const AwsCString, flags: u32) -> usize,
    pub add_host_listener: *const extern "C" fn(
        resolver: *mut AwsCHostResolver,
        options: *const AwsCHostListenerOptions,
    ) -> *mut AwsCHostListener,
    pub remove_host_listener:
        *const extern "C" fn(resolver: *mut AwsCHostResolver, listener: *mut AwsCHostListener) -> i32,
}

#[repr(C)]
pub struct AwsCHostResolver {
    pub allocator: *const AwsCAllocator,
    pub r#impl: *mut c_void,
    pub vtable: *mut AwsCHostResolverVtable,
    pub ref_count: AwsCRefCount,
    pub shutdown_options: AwsCShutdownCallbackOptions,
}

#[repr(C)]
pub struct AwsCHostResolverDefaultOptions {
    pub max_entires: usize,
    pub el_group: *mut AwsCEventLoopGroup,
    pub shutdown_options: *mut AwsCShutdownCallbackOptions,
    pub system_clock_override_fn: *const AwsCIoClockFn,
}
